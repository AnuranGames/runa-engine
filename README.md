# Runa Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

**Runa Engine** — An experimental 2D/3D game engine written in Rust, focused on performance and developer ergonomics.

> ⚠️ **Status**: Early development (Pre-Alpha). API is unstable. Not for production use.

## 🌟 Features

### ✅ Implemented

- **2D Rendering**
  - Sprites with textures and rotation
  - Automatic batching (1000+ objects = 1 draw call)
  - Transparency (alpha blending)
  - Tilemap with negative coordinate support
- **Object Component System (OCS)**
  - `Transform` component (mandatory for all objects)
  - Scripts via `Script` trait
  - Global input access (`Input::is_key_pressed()`)
- **Input System**
  - Keyboard and mouse handling
  - Screen-to-world coordinate conversion
  - `CursorInteractable` component for object clicks

### 🚧 In Progress

- [ ] World (Scene) serialization
- [ ] Object serialization
- [ ] Physics (2D/3D)
- [ ] Animations (sprites, skeletal)
- [ ] Level editor
- [ ] Custom shader support
- [x] Tilemap
- [x] **Audio** (basic)
  - [x] Sound playback via `rodio`
- [ ] **3D Support** (experimental)
  - [ ] Mesh pipeline with depth buffer
  - [ ] Instancing for massive object rendering
  - [ ] Basic lighting (diffuse + ambient)
  - [x] Simple 3D support

## 🚀 Quick Start

### Requirements

- Rust 1.75+
- GPU with Vulkan/Metal/DirectX 12 support

### Run Example

```bash
# Clone
git clone https://github.com/AnuranGames/runa-engine
cd runa-engine

# Run sandbox example
cargo run --example sandbox
```

### Create a new game project with Runa:

Create project:

```sh
cargo new my_game
cd my_game
```

Add dependencies:

```toml
[dependencies]
runa_engine = { git = "https://github.com/AnuranGames/runa-engine.git", tag = "v0.2.0-alpha.2" }
```

### Create a new game project with Runa:

#### 2D Game Example

Create project:

```sh
cargo new my_2d_game
cd my_2d_game
```

Add dependencies:

```toml
[dependencies]
runa_engine = { git = "https://github.com/AnuranGames/runa-engine.git", tag = "v0.2.0-alpha.2" }
```

### Create Your 2D Game with Player Script

```rust
// main.rs
use runa_engine::runa_app::{RunaApp, RunaWindowConfig};
use runa_engine::runa_core::World;
use runa_engine::{runa_asset, runa_core};

use runa_engine::runa_core::Vec3;
use runa_engine::runa_core::{
    components::{Camera, SpriteRenderer, Transform, ActiveCamera},
    input_system::*,
    ocs::Script,
};


fn main() {
    let mut world = World::default();
    world.spawn(Box::new(Player::new()));

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

pub struct Player {
    speed: f32,
    direction: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self { speed: 0.25, direction: Vec3::ZERO }
    }
}

impl Script for Player {
    fn construct(&self, object: &mut runa_core::ocs::Object) {
        // Add 2D orthographic camera (32x18 world units)
        object.add_component(Camera::new_ortho(32.0, 18.0, (1280, 720)));
        object.add_component(ActiveCamera);

        object
            .add_component(Transform::default())
            .add_component(SpriteRenderer {
                texture: Some(runa_asset::load_image!("assets/Charactert.png")),
            });
    }

    fn start(&mut self, object: &mut runa_core::ocs::Object) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            transform.position = Vec3::new(0.0, 0.0, 0.0);
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }

    fn update(&mut self, object: &mut runa_core::ocs::Object, _dt: f32) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            self.direction = Vec3::ZERO;
            if Input::is_key_pressed(KeyCode::KeyW) { self.direction.y = 1.0; }
            if Input::is_key_pressed(KeyCode::KeyS) { self.direction.y = -1.0; }
            if Input::is_key_pressed(KeyCode::KeyD) { self.direction.x = 1.0; }
            if Input::is_key_pressed(KeyCode::KeyA) { self.direction.x = -1.0; }
            transform.position += self.direction.normalize_or_zero() * self.speed;
        }
    }
}
```

#### 3D Game Example

For 3D games, use perspective camera:

```rust
// Add 3D perspective camera
object.add_component(Camera::new_perspective(
    Vec3::new(0.0, 0.0, 5.0), // position
    Vec3::ZERO,                // target (look at)
    Vec3::Y,                   // up
    75.0_f32.to_radians(),    // FOV
    0.1,                       // near
    1000.0,                    // far
    (1280, 720),               // viewport
));
object.add_component(ActiveCamera);

// Add 3D mesh
let mesh = Mesh::cube(2.0);
object.add_component(MeshRenderer::new(mesh));
```

For complete guides, see:

- [Creating a 2D Game](docs/tutorials/getting-started/creating-a-2d-game.md)
- [Creating a 3D Game](docs/tutorials/getting-started/creating-a-3d-game.md)

## 📂 Project Structure

```
runa-engine/
├── crates/
│   ├── runa_app/           # App entrypoint (RunaApp and WindowConfig)
│   ├── runa_assets/        # Audio system (rodio)
│   ├── runa_core/          # Core: ECS, components, scripts
│   ├── runa_editor/        # Editor and debugger for designing Runa Engine games
│   ├── runa_hub/           # Launcher for creating/managing projects
│   ├── runa_render/        # wgpu renderer
│   └── runa_render_api/    # Renderer-agnostic commands
├── examples/             # Dev tests
│   └── sandbox/            # Test sandbox
├── docs/                 # Documentation
├── CHANGELOG.md          # Changelog
├── README.md             # This file
└── Cargo.toml            # Workspace root
```

## 📜 License

Dual-licensed under:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

Choose whichever suits your project best.

## 🤝 Contributing

Project is currently private. When public:

- Open Issues for bugs and feature requests
- Submit PRs to `dev` branch
- Follow Conventional Commits

## 🙏 Acknowledgements

- **wgpu** — Cross-platform graphics API
- **glam** — Math library
- **rodio** — Audio playback

---

✨ _Built with ❤️ in Rust_ ✨
