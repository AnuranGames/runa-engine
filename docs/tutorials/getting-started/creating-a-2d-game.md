# Creating a 2D Game with Runa Engine

This guide shows you how to create a complete 2D game with Runa Engine, including player movement, sprite rendering, and mouse interaction.

## Prerequisites

- Rust 1.75+ installed
- Basic understanding of Rust
- A GPU with Vulkan, Metal, or DirectX 12 support

## Step 1: Create a New Project

```bash
cargo new my_2d_game
cd my_2d_game
```

## Step 2: Add Dependencies

Edit `Cargo.toml` to include Runa Engine:

```toml
[package]
name = "my_2d_game"
version = "0.1.0"
edition = "2021"

[dependencies]
runa_engine = { git = "https://github.com/AnuranGames/runa-engine.git", tag = "v0.2.0-alpha.2" }
```

## Step 3: Create the Project Structure

```
my_2d_game/
├── src/
│   └── main.rs
├── assets/
│   └── art/
│       └── player.png
└── Cargo.toml
```

Place a 32x32 pixel sprite at `assets/art/player.png`.

## Step 4: Create the Main File

```rust
// src/main.rs
use runa_engine::runa_app::{RunaApp, RunaWindowConfig};
use runa_engine::runa_core::World;
use runa_engine::runa_asset;
use runa_engine::runa_core::{
    components::{Camera, SpriteRenderer, Transform},
    glam::Vec3,
    input_system::*,
    ocs::Script,
};

mod player;

fn main() {
    let mut world = World::default();

    // Spawn the player with 2D camera
    world.spawn(Box::new(player::Player::new()));

    let config = RunaWindowConfig {
        title: "My 2D Game".to_string(),
        width: 1280,
        height: 720,
        fullscreen: false,
        vsync: true,
        show_fps_in_title: true,
        window_icon: None,
    };

    let _ = RunaApp::run_with_config(world, config);
}
```

## Step 5: Create the Player Script

Create `src/player.rs`:

```rust
// src/player.rs
use runa_engine::runa_asset;
use runa_engine::runa_core::{
    components::{ActiveCamera, Camera, CursorInteractable, SpriteRenderer, Transform},
    glam::{Vec2, Vec3},
    input_system::*,
    ocs::{Object, Script},
};

pub struct Player {
    speed: f32,
    direction: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            speed: 0.25,
            direction: Vec3::ZERO,
        }
    }
}

impl Script for Player {
    fn construct(&self, object: &mut Object) {
        // Add 2D orthographic camera
        // Parameters: width, height, viewport_size
        // The camera sees 32x18 world units (divided by 10 for proper scaling)
        object.add_component(Camera::new_ortho(32.0, 18.0, (1280, 720)));
        object.add_component(ActiveCamera);

        // Add transform and sprite
        object
            .add_component(Transform::default())
            .add_component(SpriteRenderer {
                texture: Some(runa_asset::load_image!("assets/art/player.png")),
            })
            // Optional: Add mouse interaction
            .add_component({
                let mut interactable = CursorInteractable::new(2.0, 2.0);
                interactable.set_on_hover_enter(|| println!("Player hovered!"));
                interactable
            });
    }

    fn start(&mut self, object: &mut Object) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            transform.position = Vec3::new(0.0, 0.0, 0.0);
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }

    fn update(&mut self, object: &mut Object, _dt: f32) {
        // Keyboard movement
        if let Some(transform) = object.get_component_mut::<Transform>() {
            self.direction = Vec3::ZERO;

            if Input::is_key_pressed(KeyCode::KeyW) {
                self.direction.y = 1.0;
            }
            if Input::is_key_pressed(KeyCode::KeyS) {
                self.direction.y = -1.0;
            }
            if Input::is_key_pressed(KeyCode::KeyD) {
                self.direction.x = 1.0;
            }
            if Input::is_key_pressed(KeyCode::KeyA) {
                self.direction.x = -1.0;
            }

            transform.position += self.direction.normalize_or_zero() * self.speed;
        }

        // Mouse drag interaction
        if let Some(ci) = object.get_component::<CursorInteractable>() {
            if ci.is_hovered && Input::is_mouse_button_pressed(MouseButton::Left) {
                if let Some(transform) = object.get_component_mut::<Transform>() {
                    if let Some(mouse_pos) = Input::get_mouse_world_position() {
                        transform.position = mouse_pos;
                    }
                }
            }
        }
    }
}
```

## Step 6: Build and Run

```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| W | Move up |
| A | Move left |
| S | Move down |
| D | Move right |
| Mouse hover + Left click | Drag player |

## Understanding the Camera

The 2D camera uses an orthographic projection:

```rust
Camera::new_ortho(width, height, viewport_size)
```

- **width/height**: The visible world size (32x18 units recommended)
- **viewport_size**: Window size in pixels (1280x720)

The engine automatically handles aspect ratio correction so sprites maintain proper proportions.

## Next Steps

- Add more game objects with [Scripts](../scripts/creating-scripts.md)
- Create levels with [Tilemaps](../tilemap/tilemap.md)
- Add sound effects with the [Audio system](../systems/audio.md)
- Explore all available [Components](../components/transform.md)

## Troubleshooting

### Black screen

Make sure you have:
1. Added `Camera::new_ortho()` component
2. Added `ActiveCamera` marker
3. Placed sprite assets in the correct folder

### Mouse position is offset

The engine handles aspect ratio automatically. Make sure you're using `Input::get_mouse_world_position()` instead of raw screen coordinates.

### Sprites are too small/large

Adjust the orthographic camera size:
- Smaller values (e.g., `16.0, 9.0`) = sprites appear larger
- Larger values (e.g., `64.0, 36.0`) = sprites appear smaller
