use std::time::Instant;

use crate::app::App;

use runa_core::components::camera2d::Camera2D;
use runa_core::input::InputState;
use runa_render_api::queue::RenderQueue;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::player::Player;
use crate::tester1::RotatingSprite1;
use crate::tester2::RotatingSprite2;
use crate::tester3::RotatingSprite3;

mod app;
mod player;
mod tester1;
mod tester2;
mod tester3;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let camera = Camera2D::new(320.0, 180.0); // виртуальный размер
    let input_state = InputState::defalut();
    let mut world = runa_core::ocs::world::World::default();

    world.spawn(Box::new(RotatingSprite2::new(5.0)));
    world.spawn(Box::new(Player::new()));

    world.construct();
    world.start();

    let mut app = App {
        last_time: Instant::now(),
        accumulator: 0.0,
        frame_count: 0,
        last_fps_update: Instant::now(),

        window: None,
        renderer: None,
        queue: RenderQueue::new(),
        camera,
        world,
        is_fullscreen: false,
        input_state,
    };

    event_loop.run_app(&mut app)
}
