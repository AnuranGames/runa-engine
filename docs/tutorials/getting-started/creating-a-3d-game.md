# Creating a 3D Game with Runa Engine

This guide shows you how to create a 3D game with Runa Engine, including 3D camera, mesh rendering, and first-person controls.

## Prerequisites

- Rust 1.75+ installed
- Basic understanding of Rust
- A GPU with Vulkan, Metal, or DirectX 12 support
- Completion of [Creating a 2D Game](creating-a-2d-game.md) recommended

## Step 1: Create a New Project

```bash
cargo new my_3d_game
cd my_3d_game
```

## Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "my_3d_game"
version = "0.1.0"
edition = "2021"

[dependencies]
runa_engine = { git = "https://github.com/AnuranGames/runa-engine.git", tag = "v0.2.0-alpha.2" }
glam = "0.32"
```

## Step 3: Create the Project Structure

```
my_3d_game/
├── src/
│   ├── main.rs
│   ├── camera_controller.rs
│   └── rotating_cube.rs
└── Cargo.toml
```

## Step 4: Create the Main File

```rust
// src/main.rs
use runa_engine::runa_app::{RunaApp, RunaWindowConfig};
use runa_engine::runa_core::World;

mod camera_controller;
mod rotating_cube;

fn main() {
    let mut world = World::default();

    // Spawn 3D camera with FPS controller
    world.spawn(Box::new(camera_controller::CameraController::new()));

    // Spawn a rotating 3D cube
    world.spawn(Box::new(rotating_cube::RotatingCube::new()));

    let config = RunaWindowConfig {
        title: "My 3D Game - WASD to move, Right-Click to look".to_string(),
        width: 1280,
        height: 720,
        fullscreen: false,
        vsync: false,
        show_fps_in_title: true,
        window_icon: None,
    };

    let _ = RunaApp::run_with_config(world, config);
}
```

## Step 5: Create the Camera Controller

Create `src/camera_controller.rs`:

```rust
// src/camera_controller.rs
use runa_engine::runa_core::{
    components::{ActiveCamera, Camera},
    glam::Vec3,
    input_system::*,
    ocs::{Object, Script},
};

static mut CURSOR_LOCKED: bool = false;

/// First-person camera controller
pub struct CameraController {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 3.0,
            sensitivity: 0.01,
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
        // Add 3D perspective camera
        object.add_component(Camera::new_perspective(
            self.position,
            self.position + Vec3::Z, // Look at point
            Vec3::Y,                  // Up vector
            75.0_f32.to_radians(),   // FOV
            0.1,                      // Near plane
            1000.0,                   // Far plane
            (1280, 720),              // Viewport size
        ));

        // Mark as active camera
        object.add_component(ActiveCamera);
    }

    fn update(&mut self, object: &mut Object, dt: f32) {
        // Toggle cursor lock on right-click
        if Input::is_mouse_button_just_pressed(MouseButton::Right) {
            unsafe {
                CURSOR_LOCKED = !CURSOR_LOCKED;
                input_system::show_cursor(!CURSOR_LOCKED);
                input_system::lock_cursor(CURSOR_LOCKED);
            }
        }

        // Mouse look (when cursor is locked)
        unsafe {
            if CURSOR_LOCKED {
                let mouse_delta = input_system::get_mouse_delta();
                self.yaw -= mouse_delta.0 * self.sensitivity;
                self.pitch -= mouse_delta.1 * self.sensitivity;
                self.pitch = self.pitch.clamp(-1.5, 1.5); // Clamp pitch
            }
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

        // Calculate look target
        let target = self.position
            + Vec3::new(
                -self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                -self.yaw.cos() * self.pitch.cos(),
            );

        // Update camera component
        if let Some(camera) = object.get_component_mut::<Camera>() {
            camera.position = self.position;
            camera.target = target;
            camera.up = Vec3::Y;
        }
    }
}
```

## Step 6: Create a Rotating 3D Cube

Create `src/rotating_cube.rs`:

```rust
// src/rotating_cube.rs
use runa_engine::runa_core::{
    components::{Mesh, MeshRenderer, Transform},
    glam::{Quat, Vec3},
    ocs::{Object, Script},
};

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
        // Add transform - position 2 units in front of camera
        object.add_component(Transform {
            position: Vec3::new(0.0, 0.0, 2.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
            previous_position: Vec3::ZERO,
            previous_rotation: Quat::IDENTITY,
        });

        // Add 3D mesh (cube)
        let mesh = Mesh::cube(2.0); // 2x2x2 cube
        object.add_component(MeshRenderer::new(mesh));
    }

    fn update(&mut self, object: &mut Object, dt: f32) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            // Rotate around Y axis
            let rotation = Quat::from_rotation_y(self.rotation_speed * dt);
            transform.rotation = transform.rotation * rotation;
        }
    }
}
```

## Step 7: Build and Run

```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| W | Move forward |
| A | Move left |
| S | Move backward |
| D | Move right |
| Space | Move up |
| Ctrl | Move down |
| Right-click (hold) | Look around with mouse |

## Understanding the 3D Camera

The 3D camera uses perspective projection:

```rust
Camera::new_perspective(
    position,    // Camera position in world
    target,      // Point to look at
    up,          // Up vector (usually Y)
    fov,         // Field of view in radians
    near,        // Near clipping plane
    far,         // Far clipping plane
    viewport_size // Window size
)
```

## Adding Textures to 3D Meshes

To add textures to your 3D meshes:

```rust
let mut mesh = Mesh::cube(2.0);
mesh.texture = Some(runa_asset::load_image!("assets/art/cube_texture.png"));
object.add_component(MeshRenderer::new(mesh));
```

## Next Steps

- Create custom 3D models with vertices and indices
- Add lighting (coming soon)
- Implement 3D physics (coming soon)
- Combine 2D and 3D rendering in the same scene

## Troubleshooting

### Black screen

Make sure you have:
1. Added `Camera::new_perspective()` component
2. Added `ActiveCamera` marker
3. Positioned objects in front of the camera (positive Z when camera looks at +Z)

### Cube is not visible

- Check that the cube is within the camera's view frustum
- Ensure near/far planes are set correctly (0.1 to 1000.0 recommended)
- Verify the cube position is not inside the camera

### Mouse look not working

- Press right-click to lock the cursor
- Check that your window has focus
- Verify `get_mouse_delta()` returns non-zero values

## Performance Tips

- Use mesh instancing for rendering many identical objects (coming soon)
- Keep draw calls low by batching materials
- Use appropriate LOD (level of detail) for distant objects
