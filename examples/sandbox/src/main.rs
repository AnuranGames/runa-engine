use runa_core::{
    components::camera2d::Camera2D,
    ocs::{object::Object, world::World},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod player;

use runa_render::renderer::{GpuContext, Renderer};

use crate::player::RotatingSprite;

fn main() {
    let _ = pollster::block_on(run());
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Runa Sandbox")
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(&window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find adapter");

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    // Создаём renderer, НО не захватываем window в closure
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");
    let context = GpuContext { device, queue };

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width.max(1),
        height: size.height.max(1),
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&context.device, &config);

    let mut renderer = Renderer::new(context, surface_format);

    let mut world = World::default();

    let mut rotating_sprite = Object::new();

    rotating_sprite.set_script(Box::new(RotatingSprite::new(10.0)));

    world.spawn(rotating_sprite);
    world.start();

    let mut render_queue = runa_render_api::queue::RenderQueue::new();

    const FIXED_TIMESTEP: f32 = 1.0 / 60.0;
    let mut accumulator = 0.0;
    let mut last_time = std::time::Instant::now();

    let window_id = window.id();

    // Больше НЕ используем &window!

    // ← ТЕПЕРЬ РАБОТАЕТ!

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                window_id: event_window_id,
                ref event,
            } if window_id == event_window_id => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {}
                WindowEvent::Resized(size) => {
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(&renderer.context.device, &config);
                }
                _ => (),
            },

            _ => (),
        }
        if let Event::AboutToWait = event {
            let current_time = std::time::Instant::now();
            let frame_time = last_time.elapsed().as_secs_f32();
            last_time = current_time;

            accumulator += frame_time;

            while accumulator >= FIXED_TIMESTEP {
                world.update(FIXED_TIMESTEP); // обновляем логику
                accumulator -= FIXED_TIMESTEP;
            }

            if let Ok(frame) = surface.get_current_texture() {
                world.render(&mut render_queue);

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = renderer.context.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Main Encoder"),
                    },
                );

                let camera = Camera2D::new(32.0, 18.0); // виртуальный размер

                renderer.draw(&mut encoder, &view, &render_queue, camera.matrix());
                renderer
                    .context
                    .queue
                    .submit(std::iter::once(encoder.finish()));
                frame.present();
            }
        }
    })?;

    Ok(())
}
