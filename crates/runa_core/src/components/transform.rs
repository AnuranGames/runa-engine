use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform {
    /// position: Vec2 { x: 0.0, y: 0.0 }, rotation: 0.0, scale: Vec2 { x: 1.0, y: 1.0 },
    pub fn default() -> Self {
        Self {
            position: Vec2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: Vec2 { x: 1.0, y: 1.0 },
        }
    }
}
