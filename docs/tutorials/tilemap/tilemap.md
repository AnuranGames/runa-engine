# Tilemap System

Tilemaps are data components used to build 2D levels.

## Composition Example

```rust
use runa_engine::runa_core::{
    components::{Rect, Tile, Tilemap, TilemapLayer, TilemapRenderer, Transform},
    glam::USizeVec2,
    ocs::Object,
};
use std::sync::Arc;

fn create_level() -> Object {
    let mut tilemap = Tilemap::centered(10, 10, USizeVec2::new(32, 32));
    let mut layer = TilemapLayer::new("Ground".to_string(), 10, 10);

    let grass = runa_engine::runa_asset::load_image!("assets/tiles/grass.png");

    for y in 0..10 {
        for x in 0..10 {
            let tile = Tile::new(
                Arc::from(grass.clone()),
                Rect::new(0.0, 0.0, 1.0, 1.0),
            );
            layer.set_tile(x, y, tile);
        }
    }

    tilemap.add_layer(layer);

    Object::new("Level")
        .with(Transform::default())
        .with(tilemap)
        .with(TilemapRenderer::new())
}
```

## Notes

- tilemap construction is data composition, not script behavior
- attach `TilemapRenderer` to render the tilemap
- use scripts only if you need runtime interaction with the tilemap after spawn
