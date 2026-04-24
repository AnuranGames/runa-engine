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
    pub primitive_hint: Option<BuiltinMeshPrimitive>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinMeshPrimitive {
    Cube,
    Quad,
    Plane,
    Pyramid,
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
            primitive_hint: Some(BuiltinMeshPrimitive::Cube),
        }
    }

    pub fn quad(width: f32, height: f32) -> Self {
        let hw = width * 0.5;
        let hh = height * 0.5;
        let vertices = vec![
            Vertex3D {
                position: [-hw, -hh, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [hw, -hh, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [hw, hh, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-hw, hh, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Self {
            vertices,
            indices,
            texture: None,
            primitive_hint: Some(BuiltinMeshPrimitive::Quad),
        }
    }

    pub fn plane(width: f32, depth: f32) -> Self {
        let hw = width * 0.5;
        let hd = depth * 0.5;
        let vertices = vec![
            Vertex3D {
                position: [-hw, 0.0, -hd],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [hw, 0.0, -hd],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [hw, 0.0, hd],
                normal: [0.0, 1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-hw, 0.0, hd],
                normal: [0.0, 1.0, 0.0],
                uv: [0.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Self {
            vertices,
            indices,
            texture: None,
            primitive_hint: Some(BuiltinMeshPrimitive::Plane),
        }
    }

    pub fn pyramid(width: f32, height: f32, depth: f32) -> Self {
        let hw = width * 0.5;
        let hd = depth * 0.5;
        let apex = [0.0, height * 0.5, 0.0];
        let base_y = -height * 0.5;
        let vertices = vec![
            Vertex3D {
                position: [-hw, base_y, -hd],
                normal: [0.0, -1.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex3D {
                position: [hw, base_y, -hd],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex3D {
                position: [hw, base_y, hd],
                normal: [0.0, -1.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: [-hw, base_y, hd],
                normal: [0.0, -1.0, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex3D {
                position: apex,
                normal: [0.0, 0.707, -0.707],
                uv: [0.5, 0.0],
            },
            Vertex3D {
                position: [-hw, base_y, -hd],
                normal: [0.0, 0.707, -0.707],
                uv: [0.0, 1.0],
            },
            Vertex3D {
                position: [hw, base_y, -hd],
                normal: [0.0, 0.707, -0.707],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: apex,
                normal: [0.707, 0.707, 0.0],
                uv: [0.5, 0.0],
            },
            Vertex3D {
                position: [hw, base_y, -hd],
                normal: [0.707, 0.707, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex3D {
                position: [hw, base_y, hd],
                normal: [0.707, 0.707, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: apex,
                normal: [0.0, 0.707, 0.707],
                uv: [0.5, 0.0],
            },
            Vertex3D {
                position: [hw, base_y, hd],
                normal: [0.0, 0.707, 0.707],
                uv: [0.0, 1.0],
            },
            Vertex3D {
                position: [-hw, base_y, hd],
                normal: [0.0, 0.707, 0.707],
                uv: [1.0, 1.0],
            },
            Vertex3D {
                position: apex,
                normal: [-0.707, 0.707, 0.0],
                uv: [0.5, 0.0],
            },
            Vertex3D {
                position: [-hw, base_y, hd],
                normal: [-0.707, 0.707, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex3D {
                position: [-hw, base_y, -hd],
                normal: [-0.707, 0.707, 0.0],
                uv: [1.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        Self {
            vertices,
            indices,
            texture: None,
            primitive_hint: Some(BuiltinMeshPrimitive::Pyramid),
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
