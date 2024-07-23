use core::panic;
use std::{fs::File, io::Write, path::PathBuf, thread, time::Duration};

use compositor_api::types::Resolution;
use compositor_pipeline::{
    audio_mixer::{AudioChannels, AudioMixingParams, AudioSamples, InputParams, MixingStrategy},
    pipeline::{
        input::{
            mp4::{Mp4Options, Source},
            InputOptions,
        },
        output::{RawAudioOptions, RawDataOutputOptions, RawVideoOptions},
        Pipeline, PipelineOutputEndCondition, RawDataReceiver, RegisterInputOptions,
        RegisterOutputOptions,
    },
    queue::{PipelineEvent, QueueInputOptions},
};
use compositor_render::{
    error::ErrorStack,
    scene::{
        Component, HorizontalAlign, InputStreamComponent, Position, RescaleMode, RescalerComponent,
        VerticalAlign,
    },
    Frame, FrameData, InputId, OutputId,
};
use crossbeam_channel::bounded;
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use integration_tests::read_rgba_texture;
use live_compositor::{
    config::{read_config, LoggerConfig, LoggerFormat},
    logger::{self, FfmpegLogLevel},
    state::ApiState,
};

// const BUNNY_FILE_URL: &str =
// "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
// const INPUT_FILE_PATH: &str = "examples/assets/call.mp4";
// const INPUT_FILE_PATH: &str = "examples/assets/testsrc.mp4";
const INPUT_FILE_PATH: &str = "examples/assets/BigBuckBunny.mp4";

// const NAME: &str = "linear";
const NAME: &str = "lanczos";
const OUTPUT_RESOLUTION: Resolution = Resolution {
    width: 441,
    height: 248,
};

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

// Start simple pipeline with output that sends PCM audio and wgpu::Textures via Rust channel.
//
// Output:
// - read content of wgpu texture and write it as png file
// - read audio samples and write raw value using debug formatting
fn main() {
    ffmpeg_next::format::network::init();
    logger::init_logger(LoggerConfig {
        ffmpeg_logger_level: FfmpegLogLevel::Info,
        format: LoggerFormat::Compact,
        level: "info,wgpu_hal=warn,wgpu_core=warn".to_string(),
    });
    let mut config = read_config();
    config.queue_options.ahead_of_time_processing = true;
    // no chromium support, so we can ignore _event_loop
    let (state, _event_loop) = ApiState::new(config).unwrap_or_else(|err| {
        panic!(
            "Failed to start compositor.\n{}",
            ErrorStack::new(&err).into_string()
        )
    });
    let output_id = OutputId("output_1".into());
    let input_id = InputId("input_id".into());

    // download_file(BUNNY_FILE_URL, INPUT_FILE_PATH).unwrap();

    let output_options = RegisterOutputOptions {
        output_options: RawDataOutputOptions {
            video: Some(RawVideoOptions {
                resolution: OUTPUT_RESOLUTION.into(),
            }),
            audio: Some(RawAudioOptions),
        },
        video: Some(compositor_pipeline::pipeline::OutputVideoOptions {
            initial: Component::Rescaler(RescalerComponent {
                id: None,
                position: Position::Static {
                    width: Some(OUTPUT_RESOLUTION.width as f32),
                    height: Some(OUTPUT_RESOLUTION.height as f32),
                },
                child: Box::new(Component::InputStream(InputStreamComponent {
                    id: None,
                    input_id: input_id.clone(),
                })),
                transition: None,
                mode: RescaleMode::Fit,
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Top,
            }),
            end_condition: PipelineOutputEndCondition::Never,
        }),
        audio: Some(compositor_pipeline::pipeline::OutputAudioOptions {
            initial: AudioMixingParams {
                inputs: vec![InputParams {
                    input_id: input_id.clone(),
                    volume: 1.0,
                }],
            },
            mixing_strategy: MixingStrategy::SumClip,
            channels: AudioChannels::Stereo,
            end_condition: PipelineOutputEndCondition::Never,
        }),
    };

    let input_options = RegisterInputOptions {
        input_options: InputOptions::Mp4(Mp4Options {
            source: Source::File(root_dir().join(INPUT_FILE_PATH)),
        }),
        queue_options: QueueInputOptions {
            required: true,
            offset: Some(Duration::ZERO),
            buffer_duration: None,
        },
    };

    Pipeline::register_input(&state.pipeline, input_id.clone(), input_options).unwrap();

    let (wgpu_device, wgpu_queue) = state.pipeline.lock().unwrap().wgpu_ctx();

    let RawDataReceiver { video, audio } = state
        .pipeline
        .lock()
        .unwrap()
        .register_raw_data_output(output_id.clone(), output_options)
        .unwrap();

    Pipeline::start(&state.pipeline);

    let (send_done, recv_done) = bounded(0);

    thread::Builder::new()
        .spawn(move || {
            for (index, frame) in video.unwrap().iter().enumerate() {
                if [0, 200, 400, 600, 800, 1000].contains(&index) {
                    write_frame(index, frame, &wgpu_device, &wgpu_queue);
                }
                if index > 1000 {
                    send_done.send(()).unwrap();
                    return;
                }
            }
        })
        .unwrap();

    let mut audio_dump =
        File::create(root_dir().join("examples/raw_channel_output_audio_dump.debug")).unwrap();

    thread::Builder::new()
        .spawn(move || {
            for packet in audio.unwrap().iter() {
                if let PipelineEvent::Data(packet) = packet {
                    let AudioSamples::Stereo(samples) = packet.samples else {
                        continue;
                    };
                    audio_dump
                        .write_all(format!("{:?} {:?}\n", packet.start_pts, samples).as_bytes())
                        .unwrap();
                } else {
                    return;
                };
            }
        })
        .unwrap();

    recv_done.recv().unwrap()
}

fn write_frame(
    index: usize,
    frame: PipelineEvent<Frame>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) {
    let PipelineEvent::Data(frame) = frame else {
        return;
    };
    let FrameData::Rgba8UnormWgpuTexture(texture) = frame.data else {
        return;
    };
    let size = texture.size();
    let frame_data = read_rgba_texture(device, queue, &texture);

    let filepath = root_dir().join(format!(
        "examples/output/{}/{}_video_frame_{}.png",
        NAME, NAME, index
    ));
    let file = File::create(filepath).unwrap();
    let encoder = PngEncoder::new(file);
    encoder
        .write_image(&frame_data, size.width, size.height, ColorType::Rgba8)
        .unwrap();
}
