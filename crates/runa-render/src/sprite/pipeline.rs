use runa_render_api::RenderCommand;

pub struct SpritePipeline {
    pipeline: wgpu::RenderPipeline,
}

impl SpritePipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        // create pipeline
        // shader, layout, etc
        todo!()
    }

    pub fn draw(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &runa_render_api::RenderQueue,
    ) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Sprite Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        for cmd in &queue.commands {
            if let RenderCommand::Sprite { texture, transform } = cmd {
                // bind texture
                // set uniforms
                // draw quad
            }
        }
    }
}
