pub mod audio;
pub mod components;
pub mod console;
pub mod debug_renderer;
pub mod input;
pub mod ocs;
pub mod renderer;
pub mod systems;

pub use ocs::world::World;

pub mod input_system {
    pub use crate::input::InputState as Input;
    pub use winit::{event::MouseButton, keyboard::KeyCode};
}
