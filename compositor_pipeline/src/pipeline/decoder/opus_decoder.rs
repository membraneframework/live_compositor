use std::{sync::Arc, time::Duration};

use compositor_render::{AudioSamples, AudioSamplesBatch, InputId};
use crossbeam_channel::Receiver;
use log::error;

use crate::{
    error::DecoderInitError,
    pipeline::structs::{AudioChannels, EncodedChunk},
    queue::Queue,
};

use super::OpusDecoderOptions;

pub struct OpusDecoder;

impl OpusDecoder {
    pub fn new(
        opts: OpusDecoderOptions,
        chunks_receiver: Receiver<EncodedChunk>,
        queue: Arc<Queue>,
        input_id: InputId,
    ) -> Result<Self, DecoderInitError> {
        let mut decoder = opus::Decoder::new(opts.sample_rate, opts.channels.into())?;

        std::thread::Builder::new()
            .name(format!("opus decoder {}", input_id.0))
            .spawn(move || {
                // Max sample rate for opus is 48kHz.
                // Usually packets contain 20ms audio chunks, but for safety we use buffer
                // that can hold >1s of 48kHz stereo audio (96k samples)
                let mut buffer = [0i16; 100_000];
                for chunk in chunks_receiver {
                    let decoded_samples_count =
                        match decoder.decode(&chunk.data, &mut buffer, false) {
                            Ok(samples_count) => samples_count,
                            Err(err) => {
                                error!("Failed to decode opus packet: {}", err);
                                continue;
                            }
                        };

                    let samples = match opts.channels {
                        AudioChannels::Mono => {
                            let samples =
                                buffer.iter().take(decoded_samples_count).cloned().collect();
                            AudioSamples::Mono(samples)
                        }
                        AudioChannels::Stereo => {
                            let mut samples = Vec::with_capacity(decoded_samples_count / 2);
                            for i in 0..(decoded_samples_count / 2) {
                                samples.push((buffer[2 * i], buffer[2 * i + 1]));
                            }
                            AudioSamples::Stereo(samples)
                        }
                    };

                    // This is RTP specific. 48000 here is a RTP clock rate.
                    let pts = Duration::from_secs_f64(f64::max(chunk.pts as f64 / 48000.0, 0.0));

                    let samples = AudioSamplesBatch {
                        samples,
                        pts,
                        sample_rate: opts.sample_rate,
                    };

                    if let Err(err) = queue.enqueue_samples(input_id.clone(), samples) {
                        error!(
                            "Error enqueueing audio samples for input {}: {}",
                            input_id, err
                        );
                    };
                }
            })
            .unwrap();

        Ok(Self)
    }
}

impl From<AudioChannels> for opus::Channels {
    fn from(value: AudioChannels) -> Self {
        match value {
            AudioChannels::Mono => opus::Channels::Mono,
            AudioChannels::Stereo => opus::Channels::Stereo,
        }
    }
}