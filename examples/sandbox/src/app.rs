use std::sync::Arc;
use std::time::Instant;

use runa_core::components::camera2d::Camera2D;
use runa_core::ocs::world::World;
use runa_render::renderer::Renderer;
use runa_render_api::queue::RenderQueue;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App<'window> {
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer<'window>>,

    pub queue: RenderQueue,
    pub camera: Camera2D,
    pub world: World,

    pub last_time: Instant,
    pub accumulator: f32,
    pub frame_count: u32,
    pub last_fps_update: Instant,
}

impl<'window> App<'window> {
    fn render(&mut self) {
        if let (Some(renderer), Some(window)) = (&mut self.renderer, &self.window) {
            // Очищаем очередь
            self.queue.clear();

            // Собираем команды
            self.world.render(&mut self.queue);

            // Рендерим
            renderer.draw(&self.queue, self.camera.matrix(), self.camera.virtual_size);

            // Обновляем FPS
            self.frame_count += 1;
            let now = Instant::now();
            if now.duration_since(self.last_fps_update).as_secs_f32() >= 1.0 {
                let fps = self.frame_count as f32
                    / now.duration_since(self.last_fps_update).as_secs_f32();
                self.frame_count = 0;
                self.last_fps_update = now;
                window.set_title(&format!("Runa Sandbox - {:.1} FPS", fps));
            }
        }
    }
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("Runa Sandbox");
            // use Arc.
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let renderer = Renderer::new(window.clone());
            self.renderer = Some(renderer);
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        const FIXED_TIMESTEP: f32 = 1.0 / 60.0;

        let current_time = Instant::now();
        let frame_time = (current_time - self.last_time).as_secs_f32().min(0.1);
        self.last_time = current_time;

        self.accumulator += frame_time;

        // Fixed timestep обновление
        while self.accumulator >= FIXED_TIMESTEP {
            self.world.update(FIXED_TIMESTEP);
            self.accumulator -= FIXED_TIMESTEP;
        }

        // Запрашиваем перерисовку
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.renderer.as_mut(), self.window.as_ref())
                {
                    wgpu_ctx.resize((new_size.width, new_size.height));
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = self.renderer.as_mut() {
                    if let Ok(frame) = wgpu_ctx.surface.get_current_texture() {
                        frame.present();
                    }
                }
                self.render();
            }
            _ => (),
        }
    }
}
