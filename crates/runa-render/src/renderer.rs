use crate::{GpuContext, RenderSettings};

pub struct Renderer<'window> {
    context: GpuContext<'window>,
    config: wgpu::SurfaceConfiguration,
}

impl<'window> Renderer<'window> {
    pub async fn new(window: &'window winit::window::Window, settings: RenderSettings) -> Self {
        let context = GpuContext::new(window).await;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: context.surface_format,
            width: settings.output_widht,
            height: settings.output_height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        context.surface.configure(&context.device, &config);

        Self { context, config }
    }

    pub fn render(&self) {
        let frame = self
            .context
            .surface
            .get_current_texture()
            .expect("Failed to acquire frame");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main Encoder"),
                });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
