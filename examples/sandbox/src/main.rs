use pollster::block_on;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use runa_render::{RenderSettings, Renderer};

use glam::Mat4;
use runa_asset::handle::Handle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Runa Sandbox")
        .build(&event_loop)
        .unwrap();

    // Создаём renderer, НО не захватываем window в closure
    let renderer = pollster::block_on(Renderer::new(&window, RenderSettings::default()));

    let window_id = window.id(); // сохраняем ID
    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            window_id: event_window_id,
            ref event,
        } if window_id == event_window_id => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => renderer.render(),
            _ => (),
        },
        _ => (),
    })?;

    Ok(())
}
