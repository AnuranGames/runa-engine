use std::sync::Arc;

use glam::{Mat4, Quat, Vec2, Vec3};

use runa_asset::TextureAsset;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct DirectionalLightData {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PointLightData {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub radius: f32,
    pub falloff: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum BackgroundModeData {
    SolidColor {
        color: Vec3,
    },
    VerticalGradient {
        zenith_color: Vec3,
        horizon_color: Vec3,
        ground_color: Vec3,
        horizon_height: f32,
        smoothness: f32,
    },
    Sky,
}

#[derive(Clone, Copy, Debug)]
pub struct AtmosphereData {
    pub ambient_color: Vec3,
    pub ambient_intensity: f32,
    pub background_intensity: f32,
    pub background: BackgroundModeData,
}

impl Default for AtmosphereData {
    fn default() -> Self {
        Self {
            ambient_color: Vec3::ONE,
            ambient_intensity: 0.15,
            background_intensity: 1.0,
            background: BackgroundModeData::VerticalGradient {
                zenith_color: Vec3::new(0.2, 0.4, 0.8),
                horizon_color: Vec3::new(0.8, 0.9, 1.0),
                ground_color: Vec3::new(0.6, 0.6, 0.7),
                horizon_height: 0.5,
                smoothness: 0.25,
            },
        }
    }
}

pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub enum RenderCommands {
    Sprite {
        texture: std::sync::Arc<TextureAsset>,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
        color: [f32; 4],
        uv_rect: [f32; 4],
        order: i32,
    },
    Text {
        text: String,
        position: Vec2,
        color: [f32; 4],
        size: f32,
    },
    DebugRect {
        position: Vec3,
        size: Vec2,
        color: [f32; 4],
    },
    Tile {
        texture: Arc<TextureAsset>,
        position: Vec3,
        size: Vec2,
        uv_rect: [f32; 4],
        flip_x: bool,
        flip_y: bool,
        color: [f32; 4],
        order: i32,
    },
    Mesh3D {
        vertices: Vec<Vertex3D>,
        indices: Vec<u32>,
        model_matrix: Mat4,
        color: [f32; 4],
        emission: [f32; 3],
        use_vertex_color: bool,
        order: i32,
        depth: f32,
    },
    // IU
    UiRect {
        rect: UiRect,
        color: [f32; 4],
        z_index: i16,
    },
    UiImage {
        texture: Arc<TextureAsset>,
        rect: UiRect,
        tint: [f32; 4],
        uv_rect: [f32; 4],
        z_index: i16,
    },
    UiText {
        text: String,
        rect: UiRect,
        color: [f32; 4],
        font_size: u16,
        z_index: i16,
    },
}
