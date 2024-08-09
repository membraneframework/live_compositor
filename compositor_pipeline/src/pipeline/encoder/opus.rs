use crossbeam_channel::{bounded, Receiver, Sender};
use log::error;
use tracing::{span, trace, warn, Level};

use crate::{
    audio_mixer::{AudioChannels, AudioSamples, OutputSamples},
    error::EncoderInitError,
    pipeline::{
        types::{EncodedChunk, EncodedChunkKind, EncoderOutputEvent},
        AudioCodec,
    },
    queue::PipelineEvent,
};

use super::AudioEncoderPreset;

#[derive(Debug, Clone)]
pub struct OpusEncoderOptions {
    pub channels: AudioChannels,
    pub preset: AudioEncoderPreset,
}

pub struct OpusEncoder {
    samples_batch_sender: Sender<PipelineEvent<OutputSamples>>,
}

impl OpusEncoder {
    pub fn new(
        options: OpusEncoderOptions,
        sample_rate: u32,
        packets_sender: Sender<EncoderOutputEvent>,
    ) -> Result<Self, EncoderInitError> {
        let (samples_batch_sender, samples_batch_receiver) = bounded(2);

        let encoder =
            opus::Encoder::new(sample_rate, options.channels.into(), options.preset.into())?;

        std::thread::Builder::new()
            .name("Opus encoder thread".to_string())
            .spawn(move || {
                let _span = span!(Level::INFO, "Opus encoder thread",).entered();
                run_encoder_thread(encoder, samples_batch_receiver, packets_sender)
            })
            .unwrap();

        Ok(Self {
            samples_batch_sender,
        })
    }

    pub fn samples_batch_sender(&self) -> &Sender<PipelineEvent<OutputSamples>> {
        &self.samples_batch_sender
    }
}

fn run_encoder_thread(
    mut encoder: opus::Encoder,
    samples_batch_receiver: Receiver<PipelineEvent<OutputSamples>>,
    packets_sender: Sender<EncoderOutputEvent>,
) {
    let mut output_buffer = [0u8; 1024 * 1024];

    let mut encode = |samples: &[i16]| match encoder.encode(samples, &mut output_buffer) {
        Ok(len) => Some(bytes::Bytes::copy_from_slice(&output_buffer[..len])),
        Err(err) => {
            error!("Opus encoding error: {}", err);
            None
        }
    };

    for msg in samples_batch_receiver {
        let batch = match msg {
            PipelineEvent::Data(batch) => batch,
            PipelineEvent::EOS => break,
        };

        let data = match batch.samples {
            AudioSamples::Mono(mono_samples) => {
                let Some(data) = encode(&mono_samples) else {
                    continue;
                };
                data
            }
            AudioSamples::Stereo(stereo_samples) => {
                let flatten_samples: Vec<i16> =
                    stereo_samples.iter().flat_map(|(l, r)| [*l, *r]).collect();
                let Some(data) = encode(&flatten_samples) else {
                    continue;
                };
                data
            }
        };
        let chunk = EncodedChunk {
            data,
            pts: batch.start_pts,
            dts: None,
            kind: EncodedChunkKind::Audio(AudioCodec::Opus),
        };

        trace!(pts=?chunk.pts, "OPUS encoder produced an encoded chunk.");
        if let Err(_err) = packets_sender.send(EncoderOutputEvent::Data(chunk)) {
            warn!("Failed to send encoded audio from OPUS encoder. Channel closed.");
            return;
        }
    }
    if let Err(_err) = packets_sender.send(EncoderOutputEvent::AudioEOS) {
        warn!("Failed to send EOS from OPUS encoder. Channel closed.")
    }
}

impl From<AudioEncoderPreset> for opus::Application {
    fn from(value: AudioEncoderPreset) -> Self {
        match value {
            AudioEncoderPreset::Quality => opus::Application::Audio,
            AudioEncoderPreset::Voip => opus::Application::Voip,
            AudioEncoderPreset::LowestLatency => opus::Application::LowDelay,
        }
    }
}
