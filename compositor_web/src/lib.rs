use std::{sync::Arc, thread, time::Duration};

use compositor_render::{
    image::{ImageSource, ImageSpec, ImageType},
    scene::{
        AbsolutePosition, Component, ComponentId, HorizontalAlign, HorizontalPosition,
        ImageComponent, InterpolationKind, Overflow, Position, RGBAColor, RescaleMode,
        RescalerComponent, ShaderComponent, Size, Transition, VerticalAlign, VerticalPosition,
        ViewChildrenDirection, ViewComponent,
    },
    shader::ShaderSpec,
    web_renderer::WebRendererInitOptions,
    Frame, FrameData, FrameSet, Framerate, OutputFrameFormat, OutputId, Renderer, RendererId,
    RendererOptions, RendererSpec, Resolution,
};
use tracing::{debug, error, info, trace, warn};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{ImageBitmap, ImageData};
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::Window};

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

struct App {
    window: Option<Arc<Window>>,
    window_ready: crossbeam_channel::Sender<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        ));

        self.window_ready
            .send(self.window.clone().unwrap())
            .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    wasm_log::init(wasm_log::Config::new(log::Level::Trace));

    Ok(())
}

#[wasm_bindgen]
pub async fn test_render() {
    use winit::platform::web::EventLoopExtWebSys;
    info!("Starting test_render");

    let event_loop = EventLoop::new().unwrap();
    let (sender, receiver) = crossbeam_channel::bounded(1);
    let mut app = App {
        window: None,
        window_ready: sender,
    };
    event_loop.spawn_app(app);

    let window = receiver.recv().unwrap();
    info!("Window ready");

    let (mut renderer, _) = Renderer::new(RendererOptions {
        web_renderer: WebRendererInitOptions {
            enable: false,
            enable_gpu: false,
        },
        framerate: Framerate { num: 30, den: 1 },
        stream_fallback_timeout: Duration::from_millis(500),
        force_gpu: false,
        wgpu_features: wgpu::Features::empty(),
        surface_target: Some(unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap() }),
    })
    .await
    .unwrap();

    info!("Renderer created");

    let img_id = RendererId("test".into());
    renderer
        .register_renderer(
            img_id.clone(),
            RendererSpec::Image(ImageSpec {
                src: ImageSource::Url {
                    url: "https://media.tenor.com/mp-OJnJhAOoAAAAe/hampter-hamster.png".into(),
                },
                image_type: ImageType::Png,
            }),
        )
        .await
        .unwrap();

    // let shader_id = RendererId("silly".into());
    // renderer
    //     .register_renderer(
    //         shader_id.clone(),
    //         RendererSpec::Shader(ShaderSpec {
    //             source: include_str!("../../integration_tests/examples/silly.wgsl").into(),
    //         }),
    //     )
    //     .await
    //     .unwrap();

    info!("Renderers registered");
    let rescaler_id = ComponentId("rescaler".into());
    let resolution = Resolution {
        width: 1280,
        height: 720,
    };
    let scene1 = Component::View(ViewComponent {
        id: None,
        children: vec![Component::Rescaler(RescalerComponent {
            id: Some(rescaler_id.clone()),
            child: Box::new(Component::Image(ImageComponent {
                id: None,
                image_id: img_id.clone(),
            })),
            position: Position::Absolute(AbsolutePosition {
                width: Some(640.0),
                height: Some(360.0),
                position_horizontal: HorizontalPosition::RightOffset(0.0),
                position_vertical: VerticalPosition::TopOffset(0.0),
                rotation_degrees: 0.0,
            }),
            transition: None,
            mode: RescaleMode::Fill,
            horizontal_align: HorizontalAlign::Right,
            vertical_align: VerticalAlign::Top,
        })],
        direction: ViewChildrenDirection::Column,
        position: Position::Absolute(AbsolutePosition {
            width: None,
            height: None,
            position_horizontal: HorizontalPosition::LeftOffset(0.0),
            position_vertical: VerticalPosition::TopOffset(0.0),
            rotation_degrees: 0.0,
        }),
        transition: None,
        overflow: Overflow::Visible,
        background_color: RGBAColor(23, 142, 33, 255),
    });
    let scene2 = Component::View(ViewComponent {
        id: None,
        children: vec![Component::Rescaler(RescalerComponent {
            id: Some(rescaler_id),
            child: Box::new(Component::Image(ImageComponent {
                id: None,
                image_id: img_id.clone(),
            })),
            position: Position::Absolute(AbsolutePosition {
                width: Some(1280.0),
                height: Some(720.0),
                position_horizontal: HorizontalPosition::RightOffset(0.0),
                position_vertical: VerticalPosition::TopOffset(0.0),
                rotation_degrees: 0.0,
            }),
            transition: Some(Transition {
                duration: Duration::from_millis(10000),
                interpolation_kind: InterpolationKind::Bounce,
            }),
            mode: RescaleMode::Fit,
            horizontal_align: HorizontalAlign::Right,
            vertical_align: VerticalAlign::Top,
        })],
        direction: ViewChildrenDirection::Column,
        position: Position::Absolute(AbsolutePosition {
            width: None,
            height: None,
            position_horizontal: HorizontalPosition::LeftOffset(0.0),
            position_vertical: VerticalPosition::TopOffset(0.0),
            rotation_degrees: 0.0,
        }),
        transition: None,
        overflow: Overflow::Visible,
        background_color: RGBAColor(23, 142, 33, 255),
    });
    // let scene = Component::Image(ImageComponent {
    //     id: None,
    //     image_id: img_id,
    // });
    // let scene = Component::Shader(ShaderComponent {
    //     id: None,
    //     children: vec![Component::Image(ImageComponent {
    //         id: None,
    //         image_id: img_id,
    //     })],
    //     shader_id,
    //     shader_param: None,
    //     size: Size {
    //         width: resolution.width as f32,
    //         height: resolution.height as f32,
    //     },
    // });
    renderer
        .update_scene(
            OutputId("output".into()),
            resolution,
            OutputFrameFormat::PlanarYuv420Bytes,
            scene1,
        )
        .unwrap();
    renderer
        .update_scene(
            OutputId("output".into()),
            resolution,
            OutputFrameFormat::PlanarYuv420Bytes,
            scene2,
        )
        .unwrap();

    info!("Scene updated");

    for it in 0..500 {
        let output_frames = renderer
            .render(FrameSet::new(Duration::from_secs_f32(0.2 * it as f32)))
            .unwrap();
        let output = output_frames
            .frames
            .get(&OutputId("output".into()))
            .unwrap();

        let resolution = output.resolution;

        info!("Scene rendered");

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        info!("Output resolution: {:?}", resolution);
        info!("Rendering to canvas");
        let data = frame_to_rgba(output);
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(data.as_slice()),
            resolution.width as u32,
            resolution.height as u32,
        )
        .unwrap();
        context.put_image_data(&data, 0.0, 0.0).unwrap();

        sleep(16).await;
    }
    //WgpuFeatures::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING| WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
}

fn frame_to_rgba(frame: &Frame) -> Vec<u8> {
    let planes = match frame.data {
        FrameData::PlanarYuv420(ref frame) => frame,
        _ => {
            panic!("Unexpected frame data");
        }
    };

    let y_plane = &planes.y_plane;
    let u_plane = &planes.u_plane;
    let v_plane = &planes.v_plane;

    // Renderer can sometimes produce resolution that is not dividable by 2
    let corrected_width = frame.resolution.width - (frame.resolution.width % 2);
    let corrected_height = frame.resolution.height - (frame.resolution.height % 2);

    let mut rgba_data = Vec::with_capacity(y_plane.len() * 4);
    for (i, y_plane) in y_plane
        .chunks(frame.resolution.width)
        .enumerate()
        .take(corrected_height)
    {
        for (j, y) in y_plane.iter().enumerate().take(corrected_width) {
            let y = (*y) as f32;
            let u = u_plane[(i / 2) * (frame.resolution.width / 2) + (j / 2)] as f32;
            let v = v_plane[(i / 2) * (frame.resolution.width / 2) + (j / 2)] as f32;

            let r = (y + 1.40200 * (v - 128.0)).clamp(0.0, 255.0);
            let g = (y - 0.34414 * (u - 128.0) - 0.71414 * (v - 128.0)).clamp(0.0, 255.0);
            let b = (y + 1.77200 * (u - 128.0)).clamp(0.0, 255.0);
            rgba_data.extend_from_slice(&[r as u8, g as u8, b as u8, 255]);
        }
    }

    rgba_data
}

pub async fn sleep(millis: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    };
    let p = js_sys::Promise::new(&mut cb);
    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}




// API POC

#[wasm_bindgen]
pub struct LiveCompositorRenderer {
    renderer: Renderer,
}

#[wasm_bindgen]
impl LiveCompositorRenderer {
    pub fn render(&self) {
        info!("Rendering");
    }

    // pub fn update_scene(&self, scene: Component) {
    //     info!("Updating scene");
    // }
}

// TODO(noituri): Consider using different camelCase
#[wasm_bindgen]
pub async fn create_renderer() -> LiveCompositorRenderer {
    use winit::platform::web::EventLoopExtWebSys;

    let event_loop = EventLoop::new().unwrap();
    let (sender, receiver) = crossbeam_channel::bounded(1);
    let mut app = App {
        window: None,
        window_ready: sender,
    };
    event_loop.spawn_app(app);

    let window = receiver.recv().unwrap();

    let (mut renderer, _) = Renderer::new(RendererOptions {
        web_renderer: WebRendererInitOptions {
            enable: false,
            enable_gpu: false,
        },
        framerate: Framerate { num: 30, den: 1 },
        stream_fallback_timeout: Duration::from_millis(500),
        force_gpu: false,
        wgpu_features: wgpu::Features::empty(),
        surface_target: Some(unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap() }),
    })
    .await
    .unwrap(); // TODO(noituri): Handle error

    LiveCompositorRenderer {
        renderer,
    }
}
