use std::sync::Arc;

use log::error;

use crate::{
    error::ErrorStack,
    state::RenderCtx,
    wgpu::{
        texture::{utils::pad_to_256, NodeTexture, RGBATexture},
        WgpuCtx,
    },
};

use super::WebRenderer;

pub struct WebRendererNode {
    renderer: Arc<WebRenderer>,
    buffers: Vec<Arc<wgpu::Buffer>>,
}

impl WebRendererNode {
    pub fn new(renderer: Arc<WebRenderer>) -> Self {
        Self {
            renderer,
            buffers: Vec::new(),
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderCtx,
        sources: &[&NodeTexture],
        target: &mut NodeTexture,
    ) {
        self.ensure_buffers(ctx.wgpu_ctx, sources);

        if let Err(err) = self.renderer.render(ctx, sources, &self.buffers, target) {
            error!(
                "Failed to run web render: {}",
                ErrorStack::new(&err).into_string()
            );
        }
    }

    fn ensure_buffers(&mut self, wgpu_ctx: &WgpuCtx, sources: &[&NodeTexture]) {
        self.buffers.resize_with(sources.len(), || {
            let buffer = wgpu_ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Temporary texture buffer"),
                size: 0,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            Arc::new(buffer)
        });

        for (texture, buffer) in sources.iter().zip(&mut self.buffers) {
            let Some(texture_state) = texture.state() else {
                continue;
            };

            let texture = texture_state.rgba_texture();
            Self::ensure_buffer_size(wgpu_ctx, buffer, texture);
        }
    }

    fn ensure_buffer_size(ctx: &WgpuCtx, buffer: &mut Arc<wgpu::Buffer>, texture: &RGBATexture) {
        let texture_size = texture.size();
        let texture_size = (4 * pad_to_256(texture_size.width) * texture_size.height) as u64;
        if buffer.size() != texture_size {
            *buffer = Arc::new(texture.new_download_buffer(ctx));
        }
    }
}
