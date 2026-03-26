use runa_core::components::Camera3D;
use runa_core::glam::Vec3;
use runa_core::input_system::get_mouse_delta;
use runa_core::input_system::{Input, KeyCode, MouseButton};
use runa_core::ocs::{Object, Script};

/// First-person camera controller for 3D navigation
///
/// Controls:
/// - WASD: Move horizontally
/// - Space: Move up
/// - Ctrl: Move down
/// - Mouse: Look around (right-click to enable)
pub struct CameraController {
    position: Vec3,
    yaw: f32,   // Horizontal rotation (radians)
    pitch: f32, // Vertical rotation (radians)
    speed: f32,
    sensitivity: f32,
    mouse_sensitivity: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0), // Start 5 units back
            yaw: 0.0,
            pitch: 0.0,
            speed: 3.0,
            sensitivity: 0.003,
            mouse_sensitivity: 0.003,
        }
    }

    fn get_forward(&self) -> Vec3 {
        Vec3::new(
            -self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        )
        .normalize()
    }

    fn get_right(&self) -> Vec3 {
        Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin()).normalize()
    }
}

impl Script for CameraController {
    fn construct(&self, object: &mut Object) {
        object.add_component(Camera3D {
            position: self.position,
            target: self.position + Vec3::Z,
            up: Vec3::Y,
            fov: 75.0_f32.to_radians(),
            near: 0.1,
            far: 1000.0,
            viewport_size: (1280, 720),
        });
    }

    fn update(&mut self, object: &mut Object, dt: f32) {
        // Mouse look (hold right-click)
        if Input::is_mouse_button_pressed(MouseButton::Right) {
            let mouse_delta = get_mouse_delta();
            self.yaw -= mouse_delta.0 * self.mouse_sensitivity;
            self.pitch += mouse_delta.1 * self.mouse_sensitivity;

            // Clamp pitch to avoid flipping
            self.pitch = self.pitch.clamp(-1.5, 1.5);
        }

        // Calculate movement direction
        let forward = self.get_forward();
        let right = self.get_right();

        let mut movement = Vec3::ZERO;

        // WASD movement
        if Input::is_key_pressed(KeyCode::KeyW) {
            movement += forward;
        }
        if Input::is_key_pressed(KeyCode::KeyS) {
            movement -= forward;
        }
        if Input::is_key_pressed(KeyCode::KeyD) {
            movement += right;
        }
        if Input::is_key_pressed(KeyCode::KeyA) {
            movement -= right;
        }

        // Vertical movement
        if Input::is_key_pressed(KeyCode::Space) {
            movement += Vec3::Y;
        }
        if Input::is_key_pressed(KeyCode::ControlLeft)
            || Input::is_key_pressed(KeyCode::ControlRight)
        {
            movement -= Vec3::Y;
        }

        // Apply movement
        if movement.length() > 0.0 {
            self.position += movement.normalize() * self.speed * dt;
        }

        // Calculate target point (where camera is looking)
        let target = self.position
            + Vec3::new(
                -self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                -self.yaw.cos() * self.pitch.cos(),
            );

        // Update camera component
        if let Some(camera) = object.get_component_mut::<Camera3D>() {
            camera.position = self.position;
            camera.target = target;
            camera.up = Vec3::Y;
        }
    }
}
