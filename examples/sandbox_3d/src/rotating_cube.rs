use runa_core::components::{Mesh, MeshRenderer, Transform};
use runa_core::glam::{Quat, Vec3};
use runa_core::ocs::{Object, Script};

/// Simple rotating 3D cube for testing
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
        // Add transform
        object.add_component(Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
            previous_position: Vec3::ZERO,
            previous_rotation: Quat::IDENTITY,
        });

        // Create a cube mesh (2x2x2 units)
        let cube_mesh = Mesh::cube(2.0);

        // Add mesh renderer
        object.add_component(MeshRenderer::new(cube_mesh));
    }

    fn update(&mut self, object: &mut Object, dt: f32) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            // Rotate around Y axis
            let rotation = Quat::from_rotation_y(self.rotation_speed * dt);
            transform.rotation = transform.rotation * rotation;
        }
    }
}
