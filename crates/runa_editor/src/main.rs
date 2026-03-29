mod content_browser;
mod editor_app;
mod editor_camera;
mod editor_settings;
mod inspector;

fn main() -> Result<(), winit::error::EventLoopError> {
    editor_app::run()
}
