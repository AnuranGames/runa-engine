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

        // Вершины куба (6 граней × 4 вершины = 24 вершины)
        let vertices = vec![
            // Передняя грань (z = h)
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
            // Задняя грань (z = -h)
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
            // Верхняя грань (y = h)
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
            // Нижняя грань (y = -h)
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
            // Правая грань (x = h)
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
            // Левая грань (x = -h)
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

        // Индексы для 12 треугольников (2 на грань × 6 граней)
        let indices = vec![
            // Передняя
            0, 1, 2, 2, 3, 0, // Задняя
            4, 5, 6, 6, 7, 4, // Верхняя
            8, 9, 10, 10, 11, 8, // Нижняя
            12, 13, 14, 14, 15, 12, // Правая
            16, 17, 18, 18, 19, 16, // Левая
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
