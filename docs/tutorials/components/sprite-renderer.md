# SpriteRenderer Component

`SpriteRenderer` displays a 2D texture.

Attach it during object composition:

```rust
use runa_engine::runa_asset::load_image;
use runa_engine::runa_core::{components::SpriteRenderer, ocs::Object};

let object = Object::new("Sprite")
    .with(SpriteRenderer::new(Some(load_image!("assets/player.png"))));
```

## Player Example

```rust
use runa_engine::runa_asset::load_image;
use runa_engine::runa_core::{
    components::{SpriteRenderer, Transform},
    ocs::{Object, Script, ScriptContext},
};

pub struct PlayerController;

impl Script for PlayerController {
    fn update(&mut self, ctx: &mut ScriptContext, dt: f32) {
        if let Some(transform) = ctx.get_component_mut::<Transform>() {
            transform.position.x += dt;
        }
    }
}

fn create_player() -> Object {
    Object::new("Player")
        .with(Transform::default())
        .with(SpriteRenderer::new(Some(load_image!("assets/player.png"))))
        .with(PlayerController)
}
```

## Notes

- PNG is the most practical default format
- attach `Transform` alongside `SpriteRenderer` for placement
- keep rendering data in components and behavior in scripts
