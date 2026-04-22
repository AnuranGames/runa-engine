use runa_asset::Handle;
use runa_asset::TextureAsset;

pub const DEFAULT_SPRITE_PIXELS_PER_UNIT: f32 = 16.0;

pub struct SpriteRenderer {
    pub texture: Option<Handle<TextureAsset>>,
    pub texture_path: Option<String>,
    // Texture size in pixels is converted into world units through this value.
    // Example: a 16px sprite at 16 PPU occupies 1 world unit before object scale.
    pub pixels_per_unit: f32,
}

impl SpriteRenderer {
    pub fn new(texture: Option<Handle<TextureAsset>>) -> Self {
        let texture_path = texture
            .as_ref()
            .map(|handle| handle.inner.path.to_string_lossy().to_string());

        Self {
            texture,
            texture_path,
            pixels_per_unit: DEFAULT_SPRITE_PIXELS_PER_UNIT,
        }
    }

    /// texture = None
    pub fn default() -> Self {
        Self {
            texture: None,
            texture_path: None,
            pixels_per_unit: DEFAULT_SPRITE_PIXELS_PER_UNIT,
        }
    }

    pub fn get_texture_handle(&self) -> Handle<TextureAsset> {
        self.texture.clone().unwrap()
    }

    pub fn set_texture(
        &mut self,
        texture: Option<Handle<TextureAsset>>,
        texture_path: Option<String>,
    ) {
        self.texture = texture;
        self.texture_path = texture_path;
    }

    pub fn pixels_per_unit(&self) -> f32 {
        self.pixels_per_unit.max(f32::EPSILON)
    }
}

