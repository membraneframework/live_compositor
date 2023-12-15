use std::sync::Arc;

use wgpu::ShaderStages;

use crate::wgpu::{
    common_pipeline::{CreateShaderError, Sampler, Vertex},
    texture::{NodeTextureState, Texture},
    WgpuCtx, WgpuErrorScope,
};

use super::embedder::RenderInfo;

#[derive(Debug)]
pub(super) struct WebRendererShader {
    pipeline: wgpu::RenderPipeline,
    texture_bgl: wgpu::BindGroupLayout,
    sampler: Sampler,
}

impl WebRendererShader {
    pub fn new(wgpu_ctx: &Arc<WgpuCtx>) -> Result<Self, CreateShaderError> {
        let scope = WgpuErrorScope::push(&wgpu_ctx.device);

        let shader_module = wgpu_ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("../web_renderer/render_website.wgsl"));
        let sampler = Sampler::new(&wgpu_ctx.device);
        let texture_bgl =
            wgpu_ctx
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Web renderer texture bgl"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                    }],
                });

        let pipeline_layout =
            wgpu_ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Web renderer pipeline layout"),
                    bind_group_layouts: &[&texture_bgl, &sampler.bind_group_layout],
                    push_constant_ranges: &[wgpu::PushConstantRange {
                        stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        range: 0..RenderInfo::size(),
                    }],
                });

        let pipeline = wgpu_ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Web renderer pipeline"),
                depth_stencil: None,
                primitive: wgpu::PrimitiveState {
                    conservative: false,
                    cull_mode: Some(wgpu::Face::Back),
                    front_face: wgpu::FrontFace::Ccw,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    unclipped_depth: false,
                },
                vertex: wgpu::VertexState {
                    buffers: &[Vertex::LAYOUT],
                    module: &shader_module,
                    entry_point: crate::wgpu::common_pipeline::VERTEX_ENTRYPOINT_NAME,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: crate::wgpu::common_pipeline::FRAGMENT_ENTRYPOINT_NAME,
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        write_mask: wgpu::ColorWrites::all(),
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    })],
                }),
                layout: Some(&pipeline_layout),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        scope.pop(&wgpu_ctx.device)?;

        Ok(Self {
            pipeline,
            texture_bgl,
            sampler,
        })
    }

    pub(super) fn render(
        &self,
        wgpu_ctx: &Arc<WgpuCtx>,
        textures: &[(Option<&Texture>, RenderInfo)],
        target: &NodeTextureState,
    ) {
        let mut encoder = wgpu_ctx.device.create_command_encoder(&Default::default());

        let mut render_plane = |(texture, render_info): &(Option<&Texture>, RenderInfo),
                                clear: bool| {
            let load = match clear {
                true => wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                false => wgpu::LoadOp::Load,
            };

            let texture_view =
                texture.map_or(&wgpu_ctx.empty_texture.view, |texture| &texture.view);

            let input_texture_bg = wgpu_ctx
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Web renderer input textures bgl"),
                    layout: &self.texture_bgl,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    }],
                });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    ops: wgpu::Operations { load, store: true },
                    view: &target.rgba_texture().texture().view,
                    resolve_target: None,
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_push_constants(ShaderStages::VERTEX_FRAGMENT, 0, &render_info.bytes());
            render_pass.set_bind_group(0, &input_texture_bg, &[]);
            render_pass.set_bind_group(1, &self.sampler.bind_group, &[]);

            wgpu_ctx.plane.draw(&mut render_pass);
        };

        for (id, render_texture) in textures.iter().enumerate() {
            render_plane(render_texture, id == 0);
        }

        wgpu_ctx.queue.submit(Some(encoder.finish()));
    }
}