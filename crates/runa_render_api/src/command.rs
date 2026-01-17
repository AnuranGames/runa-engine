use glam::Vec2;

use runa_asset::handle::Handle;
use runa_asset::texture::TextureAsset;

pub enum RenderCommands {
    Sprite {
        texture: Handle<TextureAsset>,
        position: Vec2,
        rotation: f32,
        scale: Vec2,
    },
}
