use runa_asset::TextureAsset;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub texture: Option<std::sync::Arc<TextureAsset>>,
}

impl Mesh {
    pub fn cube(size: f32) -> Self {
        let h = size * 0.5;

        // Cube vertices (6 faces x 4 vertices = 24 vertices)
        let vertices = vec![
            // Front face (z = h)
            Vertex3D {
                position: [-h, -h, h],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [h, -h, h],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [h, h, h],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-h, h, h],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0],
            },
            // Back face (z = -h)
            Vertex3D {
                position: [-h, -h, -h],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [-h, h, -h],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [h, h, -h],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [h, -h, -h],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 1.0],
            },
            // Top face (y = h)
            Vertex3D {
                position: [-h, h, -h],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [-h, h, h],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [h, h, h],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [h, h, -h],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Bottom face (y = -h)
            Vertex3D {
                position: [-h, -h, -h],
                normal: [0.0, -1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [h, -h, -h],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [h, -h, h],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-h, -h, h],
                normal: [0.0, -1.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Right face (x = h)
            Vertex3D {
                position: [h, -h, -h],
                normal: [1.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [h, h, -h],
                normal: [1.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [h, h, h],
                normal: [1.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [h, -h, h],
                normal: [1.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            // Left face (x = -h)
            Vertex3D {
                position: [-h, -h, -h],
                normal: [-1.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [-h, -h, h],
                normal: [-1.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [-h, h, h],
                normal: [-1.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-h, h, -h],
                normal: [-1.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
        ];

        // Indices for 12 triangles (2 per face x 6 faces)
        let indices = vec![
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Top
            8, 9, 10, 10, 11, 8, // Bottom
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Left
            20, 21, 22, 22, 23, 20,
        ];

        Self {
            vertices,
            indices,
            texture: None,
        }
    }
}

#[derive(Clone)]
pub struct MeshRenderer {
    pub mesh: Mesh,
    pub color: [f32; 4],
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
