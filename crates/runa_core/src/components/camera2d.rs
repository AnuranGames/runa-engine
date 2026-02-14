use glam::{Mat4, Vec2, Vec3};

/// Camera component
#[derive(Debug, Copy, Clone, Default)]
pub struct Camera2D {
    /// Local position.
    /// You can manualy set local position of many components inside your object like this.
    pub position: Vec2,
    /// Scale: 1.0 = 1:1, >1 = increase; <1 = decrease
    pub zoom: f32,
    /// Virtual size/camera render size (for example 320x180)
    pub virtual_size: Vec2,
    // pub pixel_perfect: bool, // ← новый флаг
    pub viewport_size: (u32, u32),

    aspect_correction: f32, // ← новое поле
}

impl Camera2D {
    pub fn new(vw: f32, vh: f32) -> Self {
        let mut camera = Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            virtual_size: Vec2::new(vw / 10.0, vh / 10.0),
            viewport_size: (800, 600),
            aspect_correction: 1.0,
        };
        camera.update_aspect_correction();
        camera
    }

    pub fn matrix(&self) -> Mat4 {
        let half_w = self.virtual_size.x * 0.5 / self.zoom;
        let half_h = self.virtual_size.y * 0.5 / self.zoom;

        // Инвертируем Y для WebGPU (экранная система координат)
        let proj = Mat4::orthographic_lh(
            -half_w, half_w, -half_h, half_h, // ← поменяли местами!
            -1000.0, 1000.0,
        );

        let view = Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0));
        proj * view
    }

    pub fn update_aspect_correction(&mut self) {
        let window_aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let virtual_aspect = self.virtual_size.x / self.virtual_size.y;
        self.aspect_correction = virtual_aspect / window_aspect;
    }

    pub fn screen_to_world(&self, screen_pos: (f32, f32)) -> Vec2 {
        let (screen_x, screen_y) = screen_pos;
        let (viewport_width, viewport_height) = self.viewport_size;

        // Нормализуем к NDC
        let ndc_x = (screen_x / viewport_width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_y / viewport_height as f32) * 2.0;

        // ← КЛЮЧЕВОЕ: применяем ОБРАТНУЮ коррекцию аспекта
        let corrected_ndc_x = ndc_x / self.aspect_correction;

        let world_x = corrected_ndc_x * (self.virtual_size.x * 0.5) / self.zoom + self.position.x;
        let world_y = ndc_y * (self.virtual_size.y * 0.5) / self.zoom + self.position.y;

        Vec2::new(world_x, world_y)
    }
}
