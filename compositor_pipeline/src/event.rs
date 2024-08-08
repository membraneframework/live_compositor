use compositor_render::{event_handler, InputId, OutputId};

pub(crate) enum Event {
    AudioInputStreamDelivered(InputId),
    VideoInputStreamDelivered(InputId),
    AudioInputStreamPlaying(InputId),
    VideoInputStreamPlaying(InputId),
    AudioInputStreamEos(InputId),
    VideoInputStreamEos(InputId),
    OutputEos(OutputId),
}

fn input_event(kind: &str, input_id: InputId) -> event_handler::Event {
    event_handler::Event {
        kind: kind.to_string(),
        properties: vec![("input_id".to_string(), input_id.to_string())],
    }
}

fn output_event(kind: &str, output_id: OutputId) -> event_handler::Event {
    event_handler::Event {
        kind: kind.to_string(),
        properties: vec![("output_id".to_string(), output_id.to_string())],
    }
}

impl From<Event> for event_handler::Event {
    fn from(val: Event) -> Self {
        match val {
            Event::AudioInputStreamDelivered(id) => input_event("AUDIO_INPUT_DELIVERED", id),
            Event::VideoInputStreamDelivered(id) => input_event("VIDEO_INPUT_DELIVERED", id),
            Event::AudioInputStreamPlaying(id) => input_event("AUDIO_INPUT_PLAYING", id),
            Event::VideoInputStreamPlaying(id) => input_event("VIDEO_INPUT_PLAYING", id),
            Event::AudioInputStreamEos(id) => input_event("AUDIO_INPUT_EOS", id),
            Event::VideoInputStreamEos(id) => input_event("VIDEO_INPUT_EOS", id),
            Event::OutputEos(id) => output_event("OUTPUT_EOS", id),
        }
    }
}
