use crate::math::transform::Transform;
use runa_asset::handle::Handle;
use runa_asset::image::ImageAsset;
use wgpu::Device;
use wgpu::Queue;

pub struct Sprite {
    pub transform: Transform,
    pub texture: Handle<ImageAsset>,     // CPU-side handle
    pub gpu_texture: Option<GpuTexture>, // GPU-side texture
}

pub struct Renderer {
    pub sprites: Vec<Sprite>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    // Загружаем все GPU текстуры (вызываем после инициализации GPU)
    pub fn upload_textures(&mut self, device: &Device, queue: &Queue) {
        for sprite in &mut self.sprites {
            if sprite.gpu_texture.is_none() {
                let gpu_tex = GpuTexture::from_image(device, queue, &sprite.texture.inner);
                sprite.gpu_texture = Some(gpu_tex);
            }
        }
    }

    // Пока что stub render
    pub fn render(&self) {
        // позже будем рисовать через wgpu pipeline
        // для проверки пока можно оставить пустым
    }
}
