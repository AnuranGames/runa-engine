use std::{collections::HashMap, sync::Arc};

use crate::{resources::texture::GpuTexture, sprite::pipeline::SpritePipeline};
use runa_render_api::{command::RenderCommands, queue::RenderQueue};
use wgpu::util::DeviceExt;

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    tex_coords: [f32; 2],
}

const BASE_QUAD: [Vertex; 6] = [
    Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    // Треугольник 2
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
}

pub struct Renderer {
    pub context: GpuContext,

    sprite_pipeline: SpritePipeline,

    quad_vertex_buffer: wgpu::Buffer,
    quad_vertex_count: u32,

    globals_buffer: wgpu::Buffer,

    textures: HashMap<usize, GpuTexture>,
    nearest_sampler: wgpu::Sampler,
}

impl Renderer {
    pub fn new(context: GpuContext, format: wgpu::TextureFormat) -> Self {
        let sprite_pipeline = SpritePipeline::new(&context.device, format);

        let quad_vertex_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Quad Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * 6) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quad_vertex_count = BASE_QUAD.len() as u32;

        let globals = Globals {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };

        let globals_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Globals Buffer"),
                contents: bytemuck::bytes_of(&globals),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let nearest_sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Pixel Art Sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            context,
            sprite_pipeline,
            quad_vertex_buffer,
            quad_vertex_count,
            globals_buffer,
            textures: HashMap::new(),
            nearest_sampler,
        }
    }

    pub fn draw(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &RenderQueue,
        camera_matrix: glam::Mat4,
    ) {
        self.context.queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::bytes_of(&Globals {
                view_proj: camera_matrix.to_cols_array_2d(),
            }),
        );

        for cmd in &queue.commands {
            match cmd {
                RenderCommands::Sprite {
                    texture,
                    position,
                    rotation,
                    scale,
                } => {
                    let world_vertices: Vec<Vertex> = BASE_QUAD
                        .iter()
                        .map(|v| {
                            // Применяем масштаб
                            let scaled_x = v.position[0] * scale.x;
                            let scaled_y = v.position[1] * scale.y;

                            let radrot = rotation.to_radians();
                            let cos = radrot.cos();
                            let sin = radrot.sin();

                            let rotated_x = scaled_x * cos - scaled_y * sin;
                            let rotated_y = scaled_x * sin + scaled_y * cos;

                            // Применяем позицию
                            Vertex {
                                position: [position.x + rotated_x, position.y + rotated_y],
                                tex_coords: v.tex_coords,
                            }
                        })
                        .collect();

                    // Обновляем vertex buffer
                    self.context.queue.write_buffer(
                        &self.quad_vertex_buffer,
                        0,
                        bytemuck::cast_slice(&world_vertices),
                    );

                    let key = Arc::as_ptr(&texture.inner) as usize;

                    let gpu_texture = self.textures.entry(key).or_insert_with(|| {
                        GpuTexture::from_asset(
                            &self.context.device,
                            &self.context.queue,
                            &texture.inner,
                        )
                    });

                    let bind_group =
                        self.context
                            .device
                            .create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &self.sprite_pipeline.bind_group_layout,
                                entries: &[
                                    wgpu::BindGroupEntry {
                                        binding: 0,
                                        resource: self.globals_buffer.as_entire_binding(),
                                    },
                                    wgpu::BindGroupEntry {
                                        binding: 1,
                                        resource: wgpu::BindingResource::TextureView(
                                            &gpu_texture.view,
                                        ),
                                    },
                                    wgpu::BindGroupEntry {
                                        binding: 2,
                                        resource: wgpu::BindingResource::Sampler(
                                            &self.nearest_sampler,
                                        ), // ← сэмплер!
                                    },
                                ],
                                label: Some("Sprite BindGroup"),
                            });

                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Sprite Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });

                    rpass.set_pipeline(&self.sprite_pipeline.pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    rpass.draw(0..self.quad_vertex_count, 0..1);
                }
            }
        }
    }
}
