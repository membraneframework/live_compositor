use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::{RegisterRendererError, UnregisterRendererError};

use crate::scene::{Component, OutputScene};
use crate::transformations::image_renderer::Image;
use crate::transformations::shader::Shader;
use crate::transformations::web_renderer::{self, WebRenderer};
use crate::{
    error::{InitRendererEngineError, RenderSceneError, UpdateSceneError},
    transformations::{
        text_renderer::TextRendererCtx, web_renderer::chromium_context::ChromiumContext,
    },
    types::Framerate,
    EventLoop, FrameSet, InputId, OutputId,
};
use crate::{image, OutputFrameFormat, Resolution};
use crate::{
    scene::SceneState,
    wgpu::{WgpuCtx, WgpuErrorScope},
};
use crate::{shader, RegistryType, RendererId};

use self::{
    render_graph::RenderGraph,
    render_loop::{populate_inputs, read_outputs, run_transforms},
    renderers::Renderers,
};

pub mod node;
pub mod render_graph;
mod render_loop;
pub mod renderers;

pub struct RendererOptions {
    pub web_renderer: web_renderer::WebRendererInitOptions,
    pub framerate: Framerate,
    pub stream_fallback_timeout: Duration,
    pub force_gpu: bool,
    pub wgpu_features: wgpu::Features,
}

#[derive(Clone)]
pub struct Renderer(Arc<Mutex<InnerRenderer>>);

struct InnerRenderer {
    wgpu_ctx: Arc<WgpuCtx>,
    text_renderer_ctx: TextRendererCtx,
    chromium_context: Arc<ChromiumContext>,

    render_graph: RenderGraph,
    scene: SceneState,

    renderers: Renderers,

    stream_fallback_timeout: Duration,
}

pub(crate) struct RenderCtx<'a> {
    pub(crate) wgpu_ctx: &'a Arc<WgpuCtx>,
    pub(crate) text_renderer_ctx: &'a TextRendererCtx,
    pub(crate) renderers: &'a Renderers,
    pub(crate) stream_fallback_timeout: Duration,
}

pub(crate) struct RegisterCtx {
    pub(crate) wgpu_ctx: Arc<WgpuCtx>,
    pub(crate) chromium: Arc<ChromiumContext>,
}

/// RendererSpec provides configuration necessary to construct Renderer. Renderers
/// are entities like shader, image or chromium_instance and can be used by nodes
/// to transform or generate frames.
#[derive(Debug, Clone)]
pub enum RendererSpec {
    Shader(shader::ShaderSpec),
    WebRenderer(web_renderer::WebRendererSpec),
    Image(image::ImageSpec),
}

impl Renderer {
    pub async fn new(
        opts: RendererOptions,
    ) -> Result<(Self, Arc<dyn EventLoop>), InitRendererEngineError> {
        let renderer = InnerRenderer::new(opts).await?;
        let event_loop = renderer.chromium_context.event_loop();

        Ok((Self(Arc::new(Mutex::new(renderer))), event_loop))
    }

    pub fn register_input(&self, input_id: InputId) {
        self.0.lock().unwrap().render_graph.register_input(input_id);
    }

    pub fn unregister_input(&self, input_id: &InputId) {
        self.0
            .lock()
            .unwrap()
            .render_graph
            .unregister_input(input_id);
    }

    pub fn unregister_output(&self, output_id: &OutputId) {
        self.0
            .lock()
            .unwrap()
            .render_graph
            .unregister_output(output_id);
        self.0.lock().unwrap().scene.unregister_output(output_id)
    }

    pub fn register_renderer(
        &self,
        id: RendererId,
        spec: RendererSpec,
    ) -> Result<(), RegisterRendererError> {
        let ctx = self.0.lock().unwrap().register_ctx();
        match spec {
            RendererSpec::Shader(spec) => {
                let shader = Shader::new(&ctx.wgpu_ctx, spec)
                    .map_err(|err| RegisterRendererError::Shader(err, id.clone()))?;

                let mut guard = self.0.lock().unwrap();
                Ok(guard.renderers.shaders.register(id, Arc::new(shader))?)
            }
            RendererSpec::WebRenderer(params) => {
                let web = WebRenderer::new(&ctx, &id, params)
                    .map_err(|err| RegisterRendererError::Web(err, id.clone()))?;

                let mut guard = self.0.lock().unwrap();
                Ok(guard.renderers.web_renderers.register(id, Arc::new(web))?)
            }
            RendererSpec::Image(spec) => {
                let asset = Image::new(&ctx, spec)
                    .map_err(|err| RegisterRendererError::Image(err, id.clone()))?;

                let mut guard = self.0.lock().unwrap();
                Ok(guard.renderers.images.register(id, asset)?)
            }
        }
    }

    pub fn unregister_renderer(
        &self,
        renderer_id: &RendererId,
        registry_type: RegistryType,
    ) -> Result<(), UnregisterRendererError> {
        let mut guard = self.0.lock().unwrap();
        match registry_type {
            RegistryType::Shader => guard.renderers.shaders.unregister(renderer_id)?,
            RegistryType::WebRenderer => guard.renderers.web_renderers.unregister(renderer_id)?,
            RegistryType::Image => guard.renderers.images.unregister(renderer_id)?,
        }
        Ok(())
    }

    pub fn render(&self, input: FrameSet<InputId>) -> Result<FrameSet<OutputId>, RenderSceneError> {
        self.0.lock().unwrap().render(input)
    }

    pub fn update_scene(
        &mut self,
        output_id: OutputId,
        resolution: Resolution,
        output_format: OutputFrameFormat,
        scene_root: Component,
    ) -> Result<(), UpdateSceneError> {
        self.0
            .lock()
            .unwrap()
            .update_scene(output_id, resolution, scene_root, output_format)
    }

    pub fn wgpu_ctx(&self) -> (Arc<wgpu::Device>, Arc<wgpu::Queue>) {
        let guard = self.0.lock().unwrap();
        (guard.wgpu_ctx.device.clone(), guard.wgpu_ctx.queue.clone())
    }
}

impl InnerRenderer {
    pub async fn new(opts: RendererOptions) -> Result<Self, InitRendererEngineError> {
        let wgpu_ctx = WgpuCtx::new(opts.force_gpu, opts.wgpu_features).await?;

        Ok(Self {
            wgpu_ctx: wgpu_ctx.clone(),
            text_renderer_ctx: TextRendererCtx::new(),
            chromium_context: Arc::new(ChromiumContext::new(opts.web_renderer, opts.framerate)?),
            render_graph: RenderGraph::empty(),
            renderers: Renderers::new(wgpu_ctx).await?,
            stream_fallback_timeout: opts.stream_fallback_timeout,
            scene: SceneState::new(),
        })
    }

    pub(super) fn register_ctx(&self) -> RegisterCtx {
        RegisterCtx {
            wgpu_ctx: self.wgpu_ctx.clone(),
            chromium: self.chromium_context.clone(),
        }
    }

    pub fn render(
        &mut self,
        inputs: FrameSet<InputId>,
    ) -> Result<FrameSet<OutputId>, RenderSceneError> {
        let ctx = &mut RenderCtx {
            wgpu_ctx: &self.wgpu_ctx,
            text_renderer_ctx: &self.text_renderer_ctx,
            renderers: &self.renderers,
            stream_fallback_timeout: self.stream_fallback_timeout,
        };

        let scope = WgpuErrorScope::push(&ctx.wgpu_ctx.device);

        let input_resolutions = inputs
            .frames
            .iter()
            .map(|(input_id, frame)| (input_id.clone(), frame.resolution))
            .collect();
        self.scene
            .register_render_event(inputs.pts, input_resolutions);

        let pts = inputs.pts;
        populate_inputs(ctx, &mut self.render_graph, inputs);
        run_transforms(ctx, &mut self.render_graph, pts);
        let frames = read_outputs(ctx, &mut self.render_graph, pts);

        scope.pop(&ctx.wgpu_ctx.device)?;

        Ok(FrameSet { frames, pts })
    }

    pub fn update_scene(
        &mut self,
        output_id: OutputId,
        resolution: Resolution,
        scene_root: Component,
        output_format: OutputFrameFormat,
    ) -> Result<(), UpdateSceneError> {
        let output = OutputScene {
            output_id: output_id.clone(),
            scene_root,
            resolution,
        };
        let output_node =
            self.scene
                .update_scene(output, &self.renderers, &self.text_renderer_ctx)?;
        self.render_graph.update(
            &RenderCtx {
                wgpu_ctx: &self.wgpu_ctx,
                text_renderer_ctx: &self.text_renderer_ctx,
                renderers: &self.renderers,
                stream_fallback_timeout: self.stream_fallback_timeout,
            },
            output_node,
            output_format,
        )?;
        Ok(())
    }
}
