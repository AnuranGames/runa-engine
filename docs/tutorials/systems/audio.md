# Audio System

Learn how to add sound effects and background music to your game.

## Quick Start

### Step 1: Add AudioSource Component

Add an `AudioSource` component to your object:

```rust
use runa_core::components::AudioSource;

// 2D sound (no spatial positioning)
object.add_component(AudioSource::with_asset(
    runa_asset::load_audio!("assets/sound.ogg")
));

// 3D spatial sound (affected by position and listener)
object.add_component(AudioSource::with_asset_3d(
    runa_asset::load_audio!("assets/3d_sound.ogg")
));
```

### Step 2: Add AudioListener (for 3D audio)

Add an `AudioListener` component to your camera or player:

```rust
use runa_core::components::AudioListener;

// Add to camera or player object
object.add_component(AudioListener::new());

// Or with custom stereo separation
object.add_component(AudioListener::with_stereo_separation(0.5));
```

### Step 3: Play Sounds

Play sounds in your script's `update()` method:

```rust
fn update(&mut self, object: &mut Object, dt: f32) {
    if Input::is_key_just_pressed(KeyCode::KeyV) {
        if let Some(audio) = object.get_component_mut::<AudioSource>() {
            audio.play();  // That's it!
        }
    }
}
```

## 3D Spatial Audio

The engine supports 3D positional audio with distance attenuation.

### How it works:

1. **AudioListener** — represents the "ears" (attach to camera or player)
2. **AudioSource** with `spatial = true` — sound has a position in world space
3. **Distance attenuation** — sounds get quieter with distance (inverse square law)
4. **Stereo panning** — sounds are attenuated based on left/right position (simulated)

### Example: 3D Sound Source

```rust
use runa_core::{
    components::{AudioSource, AudioListener, Transform},
    input_system::*,
    ocs::{Object, Script},
    glam::Vec3,
};

pub struct SoundEmitter;

impl Script for SoundEmitter {
    fn construct(&self, object: &mut Object) {
        object
            .add_component(Transform::default())
            .add_component(AudioSource::with_asset_3d(
                runa_asset::load_audio!("assets/ambient.ogg")
            ));
    }

    fn start(&mut self, object: &mut Object) {
        if let Some(transform) = object.get_component_mut::<Transform>() {
            transform.position = Vec3::new(5.0, 0.0, 0.0); // Position sound to the right
        }
    }

    fn update(&mut self, object: &mut Object, _dt: f32) {
        // Play ambient sound
        if let Some(audio) = object.get_component_mut::<AudioSource>() {
            if !audio.playing {
                audio.play();
            }
        }
    }
}

pub struct Player;

impl Script for Player {
    fn construct(&self, object: &mut Object) {
        object
            .add_component(Transform::default())
            .add_component(AudioListener::new()); // Player hears 3D sounds
    }

    fn update(&mut self, object: &mut Object, _dt: f32) {
        // Player movement...
    }
}
```

## Play on Awake

Automatically play sound when object spawns:

```rust
fn construct(&self, object: &mut Object) {
    let mut audio = AudioSource::with_asset(
        runa_asset::load_audio!("assets/ambient.ogg")
    );
    audio.play_on_awake = true; // Play automatically on start
    audio.looped = true; // Loop continuously
    object.add_component(audio);
}
```

## Loading Audio Files

Use the `load_audio!` macro to load sounds:

```rust
// Load an audio file (OGG or WAV format)
let sound = runa_asset::load_audio!("assets/jump.ogg");
```

## AudioSource Properties

| Property         | Type                      | Description                                             |
| ---------------- | ------------------------- | ------------------------------------------------------- |
| `audio_asset`    | `Option<Arc<AudioAsset>>` | The loaded sound data                                   |
| `volume`         | `f32`                     | Playback volume (0.0 to 1.0)                            |
| `looped`         | `bool`                    | Whether to loop the sound                               |
| `playing`        | `bool`                    | Is currently playing                                    |
| `play_requested` | `bool`                    | Request playback (set by `play()`)                      |
| `stop_requested` | `bool`                    | Request stop (set by `stop()`)                          |
| `sound_id`       | `Option<SoundId>`         | Current playing sound ID                                |
| `play_on_awake`  | `bool`                    | Play automatically when object spawns                   |
| `spatial`        | `bool`                    | Is this a 3D spatial sound                              |
| `min_distance`   | `f32`                     | Minimum distance for full volume (default: 1.0)         |
| `max_distance`   | `f32`                     | Distance at which sound becomes silent (default: 100.0) |

## AudioListener Properties

| Property            | Type   | Description                                             |
| ------------------- | ------ | ------------------------------------------------------- |
| `volume`            | `f32`  | Listener volume (0.0 to 1.0)                            |
| `active`            | `bool` | Is this listener active (only one at a time)            |
| `stereo_separation` | `f32`  | Stereo panning strength (0.0 = mono, 1.0 = full stereo) |

## Creating AudioSource

```rust
// 2D sound (no spatial positioning)
let audio = runa_asset::load_audio!("assets/sound.ogg");
let source = AudioSource::with_asset(audio);

// 3D spatial sound
let audio = runa_asset::load_audio!("assets/3d_sound.ogg");
let source = AudioSource::with_asset_3d(audio);

// Empty source (set asset later)
let source = AudioSource::new2d();
let source = AudioSource::new3d();

// Custom volume and distance
let mut source = AudioSource::with_asset_3d(audio);
source.volume = 0.5; // 50% volume
source.min_distance = 2.0; // Full volume within 2 units
source.max_distance = 50.0; // Silent beyond 50 units
```

## Playing and Stopping Sounds

```rust
// Play a sound
if let Some(audio) = object.get_component_mut::<AudioSource>() {
    audio.play();
}

// Stop a sound
if let Some(audio) = object.get_component_mut::<AudioSource>() {
    audio.stop();
}

// Check if playing
if let Some(audio) = object.get_component::<AudioSource>() {
    if audio.playing {
        println!("Sound is playing!");
    }
}
```

## Complete Example: Jump Sound

```rust
use runa_core::{
    components::{AudioSource, Transform},
    input_system::*,
    ocs::{Object, Script},
    glam::Vec3,
};

pub struct Player;

impl Player {
    pub fn new() -> Self {
        Self
    }
}

impl Script for Player {
    fn construct(&self, object: &mut Object) {
        object
            .add_component(Transform::default())
            .add_component(AudioSource::with_asset(
                runa_asset::load_audio!("assets/jump.ogg")
            ));
    }

    fn update(&mut self, object: &mut Object, _dt: f32) {
        // Play jump sound when space is pressed
        if Input::is_key_just_pressed(KeyCode::Space) {
            if let Some(audio) = object.get_component_mut::<AudioSource>() {
                audio.play();
            }
        }
    }
}
```

## Supported Formats

- **OGG** (recommended) - Good compression, open format
- **WAV** - Uncompressed, larger file size

## Tips

- Use OGG for most sounds (smaller file sizes)
- Keep sound files under 1 MB for quick loading
- Use `load_audio!` macro for automatic caching
- Sounds are played asynchronously (won't block your game)
- Call `audio.play()` as many times as you want - each call queues a new playback
- For 3D audio, always add `AudioListener` to your camera or player
- Adjust `min_distance` and `max_distance` for realistic sound falloff
- Use `stereo_separation` to control how much sounds pan left/right

## Multiple Listeners

Only **one** `AudioListener` can be active at a time. If multiple listeners exist:

- The first active listener is used
- Others are ignored

To switch listeners (e.g., for split-screen multiplayer):

```rust
// Set listener.active = false on current listener
// Set listener.active = true on new listener
```

## Note on Stereo Panning

True stereo panning (separate left/right audio channels) requires audio backend support. The current implementation uses volume attenuation based on position to simulate directionality. For true stereo panning, consider upgrading to a more advanced audio backend.

## Next Steps

- [Input](../systems/input.md) for triggering sounds
- [Scripts](../scripts/creating-scripts.md) for game logic
- [Transform](../components/transform.md) for positioning sound sources
