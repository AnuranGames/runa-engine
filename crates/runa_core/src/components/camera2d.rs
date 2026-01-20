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
}

impl Camera2D {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            virtual_size: Vec2::new(virtual_width / 10.0, virtual_height / 10.0),
            // pixel_perfect,
        }
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
}
