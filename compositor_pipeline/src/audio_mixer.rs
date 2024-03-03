use std::sync::{Arc, Mutex};

use compositor_render::{error::UpdateSceneError, OutputId};
use tracing::trace;

use self::{
    internal_audio_mixer::InternalAudioMixer,
    types::{AudioChannels, AudioMixingParams, AudioSamplesSet, OutputSamples},
};

mod internal_audio_mixer;
pub mod types;

#[derive(Debug, Clone)]
pub(super) struct AudioMixer(Arc<Mutex<InternalAudioMixer>>);

impl AudioMixer {
    pub fn new(output_sample_rate: u32) -> Self {
        Self(Arc::new(Mutex::new(InternalAudioMixer::new(
            output_sample_rate,
        ))))
    }

    pub fn mix_samples(&self, samples_set: AudioSamplesSet) -> OutputSamples {
        trace!(set=?samples_set, "Mixing samples");
        self.0.lock().unwrap().mix_samples(samples_set)
    }

    pub fn register_output(
        &self,
        output_id: OutputId,
        audio: AudioMixingParams,
        channels: AudioChannels,
    ) {
        self.0
            .lock()
            .unwrap()
            .register_output(output_id, audio, channels)
    }

    pub fn unregister_output(&self, output_id: &OutputId) {
        self.0.lock().unwrap().unregister_output(output_id)
    }

    pub fn update_output(
        &self,
        output_id: &OutputId,
        audio: AudioMixingParams,
    ) -> Result<(), UpdateSceneError> {
        self.0.lock().unwrap().update_output(output_id, audio)
    }
}
