use std::sync::Arc;

use crate::command::{AtmosphereData, DirectionalLightData, PointLightData, UiRect, Vertex3D};
use crate::RenderCommands;
use glam::{Mat4, Quat, Vec2, Vec3};
use runa_asset::TextureAsset;

#[derive(Default)]
pub struct RenderQueue {
    pub commands: Vec<RenderCommands>,
    pub directional_lights: Vec<DirectionalLightData>,
    pub point_lights: Vec<PointLightData>,
    pub atmosphere: AtmosphereData,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            directional_lights: Vec::new(),
            point_lights: Vec::new(),
            atmosphere: AtmosphereData::default(),
        }
    }

    pub fn set_atmosphere(&mut self, atmosphere: AtmosphereData) {
        self.atmosphere = atmosphere;
    }

    pub fn add_directional_light(&mut self, light: DirectionalLightData) {
        self.directional_lights.push(light);
    }

    pub fn add_point_light(&mut self, light: PointLightData) {
        self.point_lights.push(light);
    }

    pub fn draw_sprite(
        &mut self,
        texture: std::sync::Arc<TextureAsset>,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
        color: [f32; 4],
        uv_rect: [f32; 4],
        order: i32,
    ) {
        self.commands.push(RenderCommands::Sprite {
            texture,
            position,
            rotation,
            scale,
            color,
            uv_rect,
            order,
        });
    }

    pub fn draw_text(&mut self, text: String, position: Vec2, color: [f32; 4], size: f32) {
        self.commands.push(RenderCommands::Text {
            text,
            position,
            color,
            size,
        });
    }

    pub fn draw_tile(
        &mut self,
        texture: Arc<TextureAsset>,
        position: Vec3,
        size: Vec2,
        uv_rect: [f32; 4],
        flip_x: bool,
        flip_y: bool,
        color: [f32; 4],
        order: i32,
    ) {
        self.commands.push(RenderCommands::Tile {
            texture,
            position,
            size,
            uv_rect,
            flip_x,
            flip_y,
            color,
            order,
        });
    }

    pub fn draw_mesh_3d(
        &mut self,
        vertices: Vec<Vertex3D>,
        indices: Vec<u32>,
        model_matrix: Mat4,
        color: [f32; 4],
        emission: [f32; 3],
        use_vertex_color: bool,
        order: i32,
        depth: f32,
    ) {
        self.commands.push(RenderCommands::Mesh3D {
            vertices,
            indices,
            model_matrix,
            color,
            emission,
            use_vertex_color,
            order,
            depth,
        });
    }

    // UI
    pub fn draw_ui_rect(&mut self, rect: UiRect, color: [f32; 4], z_index: i16) {
        self.commands.push(RenderCommands::UiRect {
            rect,
            color,
            z_index,
        });
    }

    pub fn draw_ui_image(
        &mut self,
        texture: Arc<TextureAsset>,
        rect: UiRect,
        tint: [f32; 4],
        uv_rect: [f32; 4],
        z_index: i16,
    ) {
        self.commands.push(RenderCommands::UiImage {
            texture,
            rect,
            tint,
            uv_rect,
            z_index,
        });
    }

    pub fn draw_ui_text(
        &mut self,
        text: String,
        rect: UiRect,
        color: [f32; 4],
        font_size: u16,
        z_index: i16,
    ) {
        self.commands.push(RenderCommands::UiText {
            text,
            rect,
            color,
            font_size,
            z_index,
        });
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.directional_lights.clear();
        self.point_lights.clear();
        self.atmosphere = AtmosphereData::default();
    }
}
