use glam::{Quat, Vec2};

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,
}

impl Transform {
    /// position: Vec2::ZERO, rotation: Quat::IDENTITY, scale: Vec2::ONE,
    pub fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec2::ONE,
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation *= Quat::from_rotation_x(angle.to_radians());
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation *= Quat::from_rotation_y(angle.to_radians());
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation *= Quat::from_rotation_z(angle.to_radians());
    }
}
