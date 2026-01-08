use runa_render_api::{RenderCommand, RenderQueue};

pub struct Renderer {
    sprite_pipeline: SpritePipeline,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            sprite_pipeline: SpritePipeline::new(device, format),
        }
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &RenderQueue,
    ) {
        self.sprite_pipeline.draw(encoder, view, queue);
    }
}
