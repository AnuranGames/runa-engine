use runa_asset::Handle;
use runa_asset::TextureAsset;

pub const DEFAULT_SPRITE_PIXELS_PER_UNIT: f32 = 16.0;

pub struct SpriteRenderer {
    pub texture: Option<Handle<TextureAsset>>,
    pub texture_path: Option<String>,
    // Texture size in pixels is converted into world units through this value.
    // Example: a 16px sprite at 16 PPU occupies 1 world unit before object scale.
    pub pixels_per_unit: f32,
    // Normalized texture region: x, y, width, height.
    pub uv_rect: [f32; 4],
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
            uv_rect: Self::FULL_UV_RECT,
        }
    }

    /// texture = None
    pub fn default() -> Self {
        Self {
            texture: None,
            texture_path: None,
            pixels_per_unit: DEFAULT_SPRITE_PIXELS_PER_UNIT,
            uv_rect: Self::FULL_UV_RECT,
        }
    }

    pub const FULL_UV_RECT: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

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

    pub fn set_uv_rect(&mut self, uv_rect: [f32; 4]) {
        self.uv_rect = [
            uv_rect[0].clamp(0.0, 1.0),
            uv_rect[1].clamp(0.0, 1.0),
            uv_rect[2].clamp(0.0, 1.0),
            uv_rect[3].clamp(0.0, 1.0),
        ];
    }

    pub fn frame_size_pixels(&self) -> Option<[f32; 2]> {
        let texture = self.texture.as_ref()?;
        Some([
            texture.inner.width as f32 * self.uv_rect[2].max(f32::EPSILON),
            texture.inner.height as f32 * self.uv_rect[3].max(f32::EPSILON),
        ])
    }
}
