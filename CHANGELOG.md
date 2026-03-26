# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha.1] - 2026-03-26

### Added

- **3D Camera System**
  - `Camera3D` component with perspective projection
  - `ActiveCamera` marker component for explicit camera selection
  - Automatic camera fallback: ActiveCamera → First Camera3D → Warning
  - Safe rendering when no camera present (black screen, no crash)

- **Cursor Control API**
  - `input_system::show_cursor()` - Show/hide cursor
  - `input_system::lock_cursor()` - Lock/unlock cursor to window
  - `input_system::set_cursor_mode()` - Combined cursor control
  - Global access from anywhere in scripts

- **3D Sandbox Example**
  - `sandbox_3d` - First-person camera controller
  - WASD movement + Space/Ctrl vertical movement
  - Mouse look with locked cursor (right-click toggle)
  - Inverted Y-axis for FPS-style control

- **Input Improvements**
  - `get_mouse_delta()` now uses `DeviceEvent::MouseMotion`
  - Works correctly when cursor is locked
  - No more input lag or single-frame issues

### Changed

- **Breaking:** Camera system now requires explicit camera component
  - Removed automatic default camera creation
  - Add `Camera3D` or `Camera2D` component to enable rendering
  - Use `ActiveCamera` marker for explicit camera selection

- **Breaking:** `AudioSource::play()` API
  - Removed `world` parameter from `Script::update()`
  - Audio playback via `audio.play()` instead of `world.play_sound()`
  - `play_on_awake` flag for automatic playback

- Version bumped to 0.2.0-alpha.1 (3D rendering milestone)

### Documentation

- Updated README.md with 3D camera examples
- Added ActiveCamera usage guide
- Updated cursor control documentation
- Added troubleshooting for "No camera found" warning

## [0.1.3-alpha.1] - 2026-03-26

### Added

- **3D Spatial Audio System**
  - `AudioListener` component for camera/player
  - Distance-based volume attenuation
  - Stereo panning simulation
  - `stereo_separation` parameter

- **AudioSource Improvements**
  - `play_on_awake` flag
  - `play()` and `stop()` methods
  - `min_distance` and `max_distance` controls

- **sandbox_soundtest** example for audio testing

### Changed

- `Script::update()` signature simplified (removed `world` parameter)
- Audio playback via `AudioSource::play()` component method

## [Unreleased] %% 0.1.0 %%

### Added

- Initial project structure with workspace setup
- Core OCS system (`World`, `Object`, components)
- `Transform` component (mandatory for all objects)
- `Script` trait with lifecycle methods (`construct`, `start`, `update`)
- Global `Input` API for keyboard/mouse access anywhere in code
- 2D rendering pipeline with sprite batching (1000+ objects support)
- Tilemap component with negative coordinate support and texture batching
- `CursorInteractable` component for mouse interaction with objects
- Basic audio system using `rodio` (play/stop sounds)
- Experimental 3D mesh pipeline with depth buffer and instancing
- Camera2D with aspect ratio correction and screen-to-world conversion
- Fullscreen toggle (F11)

### Fixed

- Vertex buffer overwriting causing texture flickering in tilemaps
- Mouse world position calculation with aspect ratio correction
- Bind group caching for 10x rendering performance boost
- Z-fighting prevention in 3D pipeline (proper near/far planes)

### Changed

- Removed `input()` method from `Script` trait (replaced with global `Input` API)
- Unified texture handling: `Arc<TextureAsset>` instead of `Handle`
- Renderer now uses single vertex buffer with offsets for all draw calls
- Camera matrix calculation inverted Y for proper screen coordinates

### Deprecated

- None

### Removed

- None

### Security

- None
