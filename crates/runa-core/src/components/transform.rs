use glam::{Mat4, Vec2, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32, // only z
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: Vec2::ZERO,
            rotation: 0.0,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_translation(Vec3::new(self.position.x, self.position.y, 0.0))
            * Mat4::from_rotation_z(self.rotation)
            * Mat4::from_scale(Vec3::new(self.scale.x, self.scale.y, 1.0))
    }
}
