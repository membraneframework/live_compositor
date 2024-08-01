use std::time::Duration;

use compositor_render::{
    web_renderer::WebRendererInitOptions, Framerate, Renderer, RendererOptions,
};
use wasm_bindgen::prelude::*;
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::Window};

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
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
    // tracing_subscriber::fmt::init();
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    // let event_loop = EventLoop::new().unwrap();
    // let mut app = App { window: None };
    // event_loop.run_app(&mut app);

    Ok(())
}

#[wasm_bindgen]
pub async fn test_render() {
    let (mut renderer, _) = Renderer::new(RendererOptions {
        web_renderer: WebRendererInitOptions {
            enable: false,
            enable_gpu: false,
        },
        framerate: Framerate { num: 30, den: 1 },
        stream_fallback_timeout: Duration::from_millis(500),
        force_gpu: false,
        wgpu_features: wgpu::Features::empty(),
    })
    .await
    .unwrap();

    //WgpuFeatures::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING| WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
}
