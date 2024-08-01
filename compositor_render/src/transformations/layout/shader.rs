use std::sync::Arc;

use wgpu::ShaderStages;

use crate::wgpu::{
    common_pipeline::{self, CreateShaderError, Sampler},
    texture::{NodeTexture, NodeTextureState},
    WgpuCtx, WgpuErrorScope,
};

#[derive(Debug)]
pub struct LayoutShader {
    pipeline: wgpu::RenderPipeline,
    sampler: Sampler,
    texture_bgl: wgpu::BindGroupLayout,
}

impl LayoutShader {
    pub async fn new(wgpu_ctx: &Arc<WgpuCtx>) -> Result<Self, CreateShaderError> {
        // let scope = WgpuErrorScope::push(&wgpu_ctx.device);

        let shader_module = wgpu_ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("./apply_layouts.wgsl"));
        let result = Self::new_pipeline(wgpu_ctx, shader_module)?;

        // scope.pop_async(&wgpu_ctx.device).await?;

        Ok(result)
    }

    fn new_pipeline(
        wgpu_ctx: &Arc<WgpuCtx>,
        shader_module: wgpu::ShaderModule,
    ) -> Result<Self, CreateShaderError> {
        let sampler = Sampler::new(&wgpu_ctx.device);

        let texture_bgl = common_pipeline::create_single_texture_bgl(&wgpu_ctx.device);

        let pipeline_layout =
            wgpu_ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("shader transformation pipeline layout"),
                    bind_group_layouts: &[
                        &texture_bgl,
                        &wgpu_ctx.uniform_bgl,
                        &sampler.bind_group_layout,
                    ],
                    push_constant_ranges: &[wgpu::PushConstantRange {
                        stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        range: 0..4,
                    }],
                });

        let pipeline = common_pipeline::create_render_pipeline(
            &wgpu_ctx.device,
            &pipeline_layout,
            &shader_module,
        );

        Ok(Self {
            pipeline,
            sampler,
            texture_bgl,
        })
    }

    pub fn render(
        &self,
        wgpu_ctx: &Arc<WgpuCtx>,
        params: &wgpu::BindGroup,
        textures: &[Option<&NodeTexture>],
        target: &NodeTextureState,
    ) {
        let input_texture_bgs: Vec<wgpu::BindGroup> = self.input_textures_bg(wgpu_ctx, textures);

        let mut encoder = wgpu_ctx.device.create_command_encoder(&Default::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    view: &target.rgba_texture().texture().view,
                    resolve_target: None,
                })],
                // TODO: depth stencil attachments
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            for (layout_id, texture_bg) in input_texture_bgs.iter().enumerate() {
                render_pass.set_pipeline(&self.pipeline);

                render_pass.set_push_constants(
                    ShaderStages::VERTEX_FRAGMENT,
                    0,
                    &(layout_id as u32).to_le_bytes(),
                );

                render_pass.set_bind_group(0, texture_bg, &[]);
                render_pass.set_bind_group(1, params, &[]);
                render_pass.set_bind_group(2, &self.sampler.bind_group, &[]);

                wgpu_ctx.plane.draw(&mut render_pass);
            }
        }
        wgpu_ctx.queue.submit(Some(encoder.finish()));
    }

    fn input_textures_bg(
        &self,
        wgpu_ctx: &Arc<WgpuCtx>,
        textures: &[Option<&NodeTexture>],
    ) -> Vec<wgpu::BindGroup> {
        textures
            .iter()
            .map(|texture| {
                texture
                    .and_then(|texture| texture.state())
                    .map(|state| &state.rgba_texture().texture().view)
                    .unwrap_or(&wgpu_ctx.empty_texture.view)
            })
            .map(|view| {
                wgpu_ctx
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.texture_bgl,
                        label: None,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(view),
                        }],
                    })
            })
            .collect()
    }
}
