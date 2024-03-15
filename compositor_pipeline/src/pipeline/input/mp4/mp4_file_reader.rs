use std::{
    io::{Read, Seek},
    os::unix::fs::MetadataExt,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use bytes::{Buf, Bytes, BytesMut};
use compositor_render::InputId;
use crossbeam_channel::{Receiver, Sender};
use mp4::Mp4Reader;
use tracing::{debug, error, span, warn, Level};

use crate::{
    pipeline::{
        decoder::{AacDecoderOptions, AudioDecoderOptions, VideoDecoderOptions},
        structs::{EncodedChunk, EncodedChunkKind},
        AudioCodec, VideoCodec,
    },
    queue::PipelineEvent,
};

use super::{Mp4Error, Mp4ReaderOptions};

type ChunkReceiver = Receiver<PipelineEvent<EncodedChunk>>;

pub(crate) struct Mp4FileReader<DecoderOptions> {
    stop_thread: Arc<AtomicBool>,
    decoder_options: DecoderOptions,
}

struct TrackInfo<DecoderOptions, SampleUnpacker: FnMut(mp4::Mp4Sample) -> Bytes> {
    sample_count: u32,
    timescale: u32,
    track_id: u32,
    decoder_options: DecoderOptions,
    sample_unpacker: SampleUnpacker,
    chunk_kind: EncodedChunkKind,
}

impl Mp4FileReader<AudioDecoderOptions> {
    pub(crate) fn new_audio(
        options: Mp4ReaderOptions,
        input_id: InputId,
    ) -> Result<Option<(Self, ChunkReceiver)>, Mp4Error> {
        let stop_thread = Arc::new(AtomicBool::new(false));

        match options {
            Mp4ReaderOptions::NonFragmented { file } => {
                let input_file = std::fs::File::open(file)?;
                let size = input_file.metadata()?.size();
                Self::new(
                    input_file,
                    size,
                    Self::find_aac_info,
                    None,
                    stop_thread,
                    input_id,
                )
            }
            Mp4ReaderOptions::Fragmented {
                header,
                fragment_receiver,
            } => {
                let size = header.len() as u64;
                let reader = std::io::Cursor::new(header);
                Self::new(
                    reader,
                    size,
                    Self::find_aac_info,
                    Some(fragment_receiver),
                    stop_thread,
                    input_id,
                )
            }
        }
    }

    fn find_aac_info<Reader: Read + Seek + Send + 'static>(
        reader: &mp4::Mp4Reader<Reader>,
    ) -> Option<TrackInfo<AudioDecoderOptions, impl FnMut(mp4::Mp4Sample) -> Bytes>> {
        let Some((&track_id, track, aac)) = reader.tracks().iter().find_map(|(id, track)| {
            let track_type = track.track_type().ok()?;

            let media_type = track.media_type().ok()?;

            let aac = track.trak.mdia.minf.stbl.stsd.mp4a.as_ref();

            if track_type != mp4::TrackType::Audio
                || media_type != mp4::MediaType::AAC
                || aac.is_none()
            {
                return None;
            }

            aac.map(|aac| (id, track, aac))
        }) else {
            return None;
        };

        let asc = aac
            .esds
            .as_ref()
            .and_then(|esds| esds.es_desc.dec_config.dec_specific.full_config.clone())
            .map(Bytes::from);

        let decoder_options = AudioDecoderOptions::Aac(AacDecoderOptions { asc });

        Some(TrackInfo {
            sample_count: track.sample_count(),
            timescale: track.timescale(),
            track_id,
            decoder_options,
            sample_unpacker: |sample| sample.bytes,
            chunk_kind: EncodedChunkKind::Audio(AudioCodec::Aac),
        })
    }
}

impl Mp4FileReader<VideoDecoderOptions> {
    pub(crate) fn new_video(
        options: Mp4ReaderOptions,
        input_id: InputId,
    ) -> Result<Option<(Mp4FileReader<VideoDecoderOptions>, ChunkReceiver)>, Mp4Error> {
        let stop_thread = Arc::new(AtomicBool::new(false));

        match options {
            Mp4ReaderOptions::NonFragmented { file } => {
                let input_file = std::fs::File::open(file)?;
                let size = input_file.metadata()?.size();
                Self::new(
                    input_file,
                    size,
                    Self::find_h264_info,
                    None,
                    stop_thread,
                    input_id,
                )
            }
            Mp4ReaderOptions::Fragmented {
                header,
                fragment_receiver,
            } => {
                let size = header.len() as u64;
                let reader = std::io::Cursor::new(header);
                Self::new(
                    reader,
                    size,
                    Self::find_h264_info,
                    Some(fragment_receiver),
                    stop_thread,
                    input_id,
                )
            }
        }
    }

    fn find_h264_info<Reader: Read + Seek + Send + 'static>(
        reader: &mp4::Mp4Reader<Reader>,
    ) -> Option<TrackInfo<VideoDecoderOptions, impl FnMut(mp4::Mp4Sample) -> Bytes>> {
        let Some((&track_id, track, avc)) = reader.tracks().iter().find_map(|(id, track)| {
            let track_type = track.track_type().ok()?;

            let media_type = track.media_type().ok()?;

            let avc = track.avc1_or_3_inner();

            if track_type != mp4::TrackType::Video
                || media_type != mp4::MediaType::H264
                || avc.is_none()
            {
                return None;
            }

            avc.map(|avc| (id, track, avc))
        }) else {
            return None;
        };

        // sps and pps have to be extracted from the container, interleaved with [0, 0, 0, 1],
        // concatenated and prepended to the first frame.
        let sps = avc
            .avcc
            .sequence_parameter_sets
            .iter()
            .flat_map(|s| [0, 0, 0, 1].iter().chain(s.bytes.iter()));

        let pps = avc
            .avcc
            .picture_parameter_sets
            .iter()
            .flat_map(|s| [0, 0, 0, 1].iter().chain(s.bytes.iter()));

        let mut sps_and_pps_payload = Some(sps.chain(pps).copied().collect::<Bytes>());

        let length_size = avc.avcc.length_size_minus_one + 1;

        let sample_unpacker = move |sample: mp4::Mp4Sample| {
            let mut sample_data = sample.bytes.reader();
            let mut data: BytesMut = Default::default();

            if let Some(first_nal) = sps_and_pps_payload.take() {
                data.extend_from_slice(&first_nal);
            }

            // the mp4 sample contains one h264 access unit (possibly more than one NAL).
            // the NALs are stored as: <length_size bytes long big endian encoded length><the NAL>.
            // we need to convert this into Annex B, in which NALs are separated by
            // [0, 0, 0, 1]. `lenght_size` is at most 4 bytes long.
            loop {
                let mut len = [0u8; 4];

                if sample_data
                    .read_exact(&mut len[4 - length_size as usize..])
                    .is_err()
                {
                    break;
                }

                let len = u32::from_be_bytes(len);

                let mut nalu = bytes::BytesMut::zeroed(len as usize);
                sample_data.read_exact(&mut nalu).unwrap();

                data.extend_from_slice(&[0, 0, 0, 1]);
                data.extend_from_slice(&nalu);
            }

            data.freeze()
        };

        let decoder_options = VideoDecoderOptions {
            codec: VideoCodec::H264,
        };

        Some(TrackInfo {
            sample_count: track.sample_count(),
            timescale: track.timescale(),
            decoder_options,
            track_id,
            sample_unpacker,
            chunk_kind: EncodedChunkKind::Video(VideoCodec::H264),
        })
    }
}

impl<DecoderOptions: Clone + Send + 'static> Mp4FileReader<DecoderOptions> {
    fn new<
        Reader: Read + Seek + Send + 'static,
        SampleUnpacker: FnMut(mp4::Mp4Sample) -> Bytes + Send + 'static,
    >(
        reader: Reader,
        size: u64,
        track_info_reader: impl Fn(
            &mp4::Mp4Reader<Reader>,
        ) -> Option<TrackInfo<DecoderOptions, SampleUnpacker>>,
        fragment_receiver: Option<Receiver<PipelineEvent<Bytes>>>,
        stop_thread: Arc<AtomicBool>,
        input_id: InputId,
    ) -> Result<Option<(Self, ChunkReceiver)>, Mp4Error> {
        let reader = mp4::Mp4Reader::read_header(reader, size)?;

        let Some(track_info) = track_info_reader(&reader) else {
            return Ok(None);
        };

        let (sender, receiver) = crossbeam_channel::bounded(10);

        let stop_thread_clone = stop_thread.clone();
        let decoder_options = track_info.decoder_options.clone();

        std::thread::Builder::new()
            .name(format!("mp4 reader {input_id}"))
            .spawn(move || {
                let _span = span!(Level::INFO, "MP4", input_id = input_id.to_string()).entered();
                run_reader_thread(
                    reader,
                    sender,
                    stop_thread_clone,
                    fragment_receiver,
                    track_info,
                );
                debug!("Closing MP4 reader thread");
            })
            .unwrap();

        Ok(Some((
            Mp4FileReader {
                stop_thread,
                decoder_options,
            },
            receiver,
        )))
    }

    pub(crate) fn decoder_options(&self) -> DecoderOptions {
        self.decoder_options.clone()
    }
}

impl<D> Drop for Mp4FileReader<D> {
    fn drop(&mut self) {
        self.stop_thread
            .store(true, std::sync::atomic::Ordering::Relaxed)
    }
}

fn run_reader_thread<Reader: Read + Seek, DecoderOptions>(
    mut reader: Mp4Reader<Reader>,
    sender: Sender<PipelineEvent<EncodedChunk>>,
    stop_thread: Arc<AtomicBool>,
    fragment_receiver: Option<Receiver<PipelineEvent<Bytes>>>,
    track_info: TrackInfo<DecoderOptions, impl FnMut(mp4::Mp4Sample) -> Bytes>,
) {
    let mut sample_unpacker = track_info.sample_unpacker;

    read_samples(
        &mut reader,
        track_info.track_id,
        track_info.sample_count,
        track_info.timescale,
        track_info.chunk_kind,
        &mut sample_unpacker,
        &stop_thread,
        &sender,
    );

    if let Some(fragment_receiver) = fragment_receiver {
        for fragment in fragment_receiver.into_iter() {
            let fragment = match fragment {
                PipelineEvent::Data(fragment) => fragment,
                PipelineEvent::EOS => break,
            };

            let size = fragment.len() as u64;
            let fragment_reader =
                reader.read_fragment_header(std::io::Cursor::new(&fragment), size);

            let mut fragment_reader = match fragment_reader {
                Ok(fragment_reader) => fragment_reader,
                Err(e) => {
                    error!("Error while reading an mp4 fragment: {e}");
                    continue;
                }
            };

            let sample_count = fragment_reader
                .tracks()
                .get(&track_info.track_id)
                .unwrap()
                .sample_count();

            read_samples(
                &mut fragment_reader,
                track_info.track_id,
                sample_count,
                track_info.timescale,
                track_info.chunk_kind,
                &mut sample_unpacker,
                &stop_thread,
                &sender,
            );
        }
    }

    if let Err(_err) = sender.send(PipelineEvent::EOS) {
        debug!("Failed to send EOS from MP4 video reader. Channel closed.");
    }
}

#[allow(clippy::too_many_arguments)]
fn read_samples<R: Read + Seek, F: FnMut(mp4::Mp4Sample) -> Bytes>(
    reader: &mut mp4::Mp4Reader<R>,
    track_id: u32,
    sample_count: u32,
    timescale: u32,
    chunk_kind: EncodedChunkKind,
    sample_unpacker: &mut F,
    stop_thread: &AtomicBool,
    sender: &Sender<PipelineEvent<EncodedChunk>>,
) {
    for i in 1..sample_count {
        if stop_thread.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        match reader.read_sample(track_id, i) {
            Ok(Some(sample)) => {
                let rendering_offset = sample.rendering_offset;
                let start_time = sample.start_time;
                let data = sample_unpacker(sample);

                let dts = Duration::from_secs_f64(start_time as f64 / timescale as f64);
                let chunk = EncodedChunk {
                    data,
                    pts: Duration::from_secs_f64(
                        (start_time as f64 + rendering_offset as f64) / timescale as f64,
                    ),
                    dts: Some(dts),
                    kind: chunk_kind,
                };

                match sender.send(PipelineEvent::Data(chunk)) {
                    Ok(_) => {}
                    Err(_) => {
                        debug!("Channel disconnected.");
                        return;
                    }
                }
            }
            Err(e) => {
                warn!("Error while reading MP4 video sample: {:?}", e);
            }
            _ => {}
        }
    }
}
