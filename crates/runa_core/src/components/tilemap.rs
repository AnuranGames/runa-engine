use glam::{IVec2, USizeVec2, Vec3};
use runa_asset::TextureAsset;
use std::sync::Arc;

/// Rectangle used for UV coordinates and placement.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// A single tile in a tilemap.
#[derive(Clone)]
pub struct Tile {
    pub texture: Option<Arc<TextureAsset>>, // None means an empty tile
    pub uv_rect: Rect,                      // Part of the texture atlas
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Tile {
    pub fn new(texture: Arc<TextureAsset>, uv_rect: Rect) -> Self {
        Self {
            texture: Some(texture),
            uv_rect,
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            texture: None,
            uv_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            flip_x: false,
            flip_y: false,
        }
    }
}

/// A single tilemap layer.
#[derive(Clone)]
pub struct TilemapLayer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>, // width * height elements
    pub visible: bool,
    pub opacity: f32,
}

impl TilemapLayer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            width,
            height,
            tiles: vec![Tile::empty(); (width * height) as usize],
            visible: true,
            opacity: 1.0,
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        let index = (y * self.width + x) as usize;
        if index < self.tiles.len() {
            self.tiles[index] = tile;
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        let index = (y * self.width + x) as usize;
        self.tiles.get(index)
    }
}

/// Tilemap data component.
#[derive(Clone)]
pub struct Tilemap {
    /// Map size in tiles.
    pub width: u32,
    pub height: u32,

    /// Tile size in world units.
    /// Example: 16 pixels at pixels_per_unit=16 -> tile_size=1.0
    pub tile_size: USizeVec2,
    pub offset: IVec2,

    /// Layers ordered from back to front.
    pub layers: Vec<TilemapLayer>,
}

impl Tilemap {
    /// Creates a map centered at world origin.
    pub fn centered(width: u32, height: u32, tile_size: USizeVec2) -> Self {
        let offset = IVec2::new(-(width as i32) / 2, -(height as i32) / 2);

        Self {
            width,
            height,
            tile_size,
            offset,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: TilemapLayer) {
        self.layers.push(layer);
    }

    /// Sets a tile using world tile coordinates, which may be negative.
    pub fn set_tile(&mut self, world_x: i32, world_y: i32, tile: Tile) {
        // Convert world coordinates to array indices
        let array_x = (world_x - self.offset.x) as u32;
        let array_y = (world_y - self.offset.y) as u32;

        // Bounds check
        if array_x < self.width && array_y < self.height {
            for layer in &mut self.layers {
                layer.set_tile(array_x, array_y, tile.clone());
            }
        }
    }

    /// Converts world coordinates to tile coordinates.
    pub fn world_to_tile(&self, world_pos: Vec3) -> (i32, i32) {
        let tile_x = (world_pos.x / self.tile_size.x as f32).floor() as i32;
        let tile_y = (world_pos.y / self.tile_size.y as f32).floor() as i32;
        (tile_x, tile_y)
    }

    /// Converts tile coordinates to the world-space tile center.
    pub fn tile_to_world(&self, tile_x: i32, tile_y: i32) -> Vec3 {
        Vec3::new(
            (tile_x as f32) * self.tile_size.x as f32,
            (tile_y as f32) * self.tile_size.y as f32,
            0.0,
        )
    }
}

/// Rendering Component for Tilemap
#[derive(Clone)]
pub struct TilemapRenderer;

impl TilemapRenderer {
    pub fn new() -> Self {
        Self
    }
}
