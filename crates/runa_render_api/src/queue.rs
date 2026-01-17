use crate::command::RenderCommands;
use glam::Vec2;
use runa_asset::{handle::Handle, texture::TextureAsset};

pub struct RenderQueue {
    pub commands: Vec<RenderCommands>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn draw_sprite(
        &mut self,
        texture: Handle<TextureAsset>,
        position: Vec2,
        rotation: f32,
        scale: Vec2,
    ) {
        self.commands.push(RenderCommands::Sprite {
            texture,
            position,
            rotation,
            scale,
        });
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
