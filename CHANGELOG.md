# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3-alpha.1] - 2026-03-26

### Added

- **3D Spatial Audio System**
  - `AudioListener` component for camera/player (represents "ears" of audio system)
  - Distance-based volume attenuation (inverse square law)
  - Stereo panning based on sound position relative to listener
  - `stereo_separation` parameter (0.0 = mono, 1.0 = full stereo)
  - Dynamic volume updates without sound restart (no artifacts!)
  - `AudioSource::new3d()` and `AudioSource::with_asset_3d()` constructors
  - `min_distance` and `max_distance` for sound attenuation control
- **AudioSource Improvements**
  - `play_on_awake` flag for automatic playback on spawn
  - `play()` and `stop()` methods on AudioSource component
  - Internal flag-based system (`play_requested`, `stop_requested`)
  - Sound ID tracking for managing playing sounds
- **Object System**
  - `Object::get_world_mut()` method for accessing world from object
  - Internal `world_ptr` for system access (safe raw pointer usage)
- **Script System**
  - Simplified `Script::update()` signature (removed `world` parameter)
  - Audio playback via `audio.play()` instead of `world.play_sound()`
- **Examples**
  - `sandbox_soundtest` example demonstrating 3D audio with left/right emitters
  - Test scene with two spatial sound sources at different positions

### Fixed

- Audio artifacts from sound restart - now uses `Player::set_volume()` for smooth volume changes
- Borrow checker issues in `World::update()` with audio processing
- Component import paths in examples

### Changed

- **Breaking:** `Script::update(&mut self, object: &mut Object, dt: f32)` - removed `world` parameter
- **Breaking:** Audio playback now via `AudioSource::play()` instead of `World::play_sound()`
- AudioEngine now updates spatial volumes every frame without restarting sounds
- `AudioListener` component required for 3D audio (auto-added to player/camera)
- Only one active `AudioListener` at a time (first active is used)

### Documentation

- Updated `docs/tutorials/systems/audio.md` with 3D audio guide
- Added examples for `AudioListener` setup
- Documented `stereo_separation` parameter
- Updated all tutorial examples to new API

### Technical Notes

- True stereo panning limited by rodio 0.22 (no `SpatialSink`)
- Current implementation uses volume attenuation for directionality simulation
- Future: upgrade audio backend for true left/right channel panning

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
