use std::sync::{Arc, Mutex};

use compositor_render::InputId;

use crate::{error::RegisterInputError, Pipeline};

use super::{decoder, input, Port, RegisterInputOptions};

pub struct PipelineInput {
    pub input: input::Input,

    /// Some(received) - Whether EOS was received from queue on audio stream for that input.
    /// None - No audio configured for that input.
    pub(super) audio_eos_received: Option<bool>,
    /// Some(received) - Whether EOS was received from queue on video stream for that input.
    /// None - No video configured for that input.
    pub(super) video_eos_received: Option<bool>,
}

pub(super) fn register_pipeline_input(
    pipeline: &Arc<Mutex<Pipeline>>,
    input_id: InputId,
    register_options: RegisterInputOptions,
) -> Result<Option<Port>, RegisterInputError> {
    let RegisterInputOptions {
        input_options,
        queue_options,
    } = register_options;
    let (download_dir, output_sample_rate) = {
        let guard = pipeline.lock().unwrap();
        if guard.inputs.contains_key(&input_id) {
            return Err(RegisterInputError::AlreadyRegistered(input_id));
        }
        (guard.download_dir.clone(), guard.output_sample_rate)
    };

    let (input, chunks_receiver, decoder_options, port) =
        input::Input::new(input_options, &input_id, &download_dir)
            .map_err(|e| RegisterInputError::InputError(input_id.clone(), e))?;

    let (audio_eos_received, video_eos_received) = (
        decoder_options.audio.as_ref().map(|_| false),
        decoder_options.video.as_ref().map(|_| false),
    );
    let decoded_data_receiver = decoder::start_decoder(
        input_id.clone(),
        chunks_receiver,
        decoder_options,
        output_sample_rate,
    )
    .map_err(|e| RegisterInputError::DecoderError(input_id.clone(), e))?;

    let pipeline_input = PipelineInput {
        input,
        audio_eos_received,
        video_eos_received,
    };

    let mut guard = pipeline.lock().unwrap();

    if pipeline_input.audio_eos_received.is_some() {
        for (_, output) in guard.outputs.iter_mut() {
            if let Some(ref mut cond) = output.audio_end_condition {
                cond.on_input_registered(&input_id);
            }
        }
    }

    if pipeline_input.video_eos_received.is_some() {
        for (_, output) in guard.outputs.iter_mut() {
            if let Some(ref mut cond) = output.video_end_condition {
                cond.on_input_registered(&input_id);
            }
        }
    }

    guard.inputs.insert(input_id.clone(), pipeline_input);
    guard
        .queue
        .add_input(&input_id, decoded_data_receiver, queue_options);
    guard.renderer.register_input(input_id);

    Ok(port)
}

impl PipelineInput {
    pub(super) fn on_audio_eos(&mut self) {
        self.audio_eos_received = self.audio_eos_received.map(|_| true);
    }
    pub(super) fn on_video_eos(&mut self) {
        self.audio_eos_received = self.audio_eos_received.map(|_| true);
    }
}
