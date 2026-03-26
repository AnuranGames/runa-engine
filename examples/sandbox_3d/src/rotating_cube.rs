use runa_core::components::{SpriteRenderer, Transform};
use runa_core::glam::{Quat, Vec3};
use runa_core::ocs::{Object, Script};

/// Simple test sprite to verify rendering works
pub struct RotatingCube {
    rotation_speed: f32,
}

impl RotatingCube {
    pub fn new() -> Self {
        Self {
            rotation_speed: 0.5, // radians per second
        }
    }
}

impl Script for RotatingCube {
    fn construct(&self, object: &mut Object) {
        // Add transform - position 3 units in front of camera
        object.add_component(Transform {
            position: Vec3::new(0.0, 0.0, 2.0), // 2 units in front of camera at (0,0,5)
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
            previous_position: Vec3::ZERO,
            previous_rotation: Quat::IDENTITY,
        });

        // Add a simple sprite (white texture will be visible)
        object.add_component(SpriteRenderer {
            texture: Some(runa_asset::load_image!("assets/art/Tester1.png")),
        });
    }

    fn update(&mut self, object: &mut Object, dt: f32) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            // Rotate around Y axis
            let rotation = Quat::from_rotation_y(self.rotation_speed * dt);
            transform.rotation = transform.rotation * rotation;
        }
    }
}
