use glam::Mat4;
use runa_asset::handle::Handle;
use runa_asset::image::ImageAsset;

pub enum RenderCommand {
    Sprite {
        texture: Handle<ImageAsset>,
        transform: Mat4,
    },
}

pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
