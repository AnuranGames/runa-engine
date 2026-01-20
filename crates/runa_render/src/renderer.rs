use std::{collections::HashMap, sync::Arc};

use crate::{resources::texture::GpuTexture, sprite::pipeline::SpritePipeline};
use glam::{Vec2, Vec3};
use runa_render_api::{command::RenderCommands, queue::RenderQueue};
use wgpu::util::DeviceExt;
use wgpu::{MemoryHints::Performance, Trace};
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
    pub aspect: f32,
    pub _padding: [f32; 7], // выравнивание до 16 байт
}

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    sprite_pipeline: SpritePipeline,

    quad_vertex_buffer: wgpu::Buffer,
    quad_vertex_count: u32,

    globals_buffer: wgpu::Buffer,

    textures: HashMap<usize, GpuTexture>,
    nearest_sampler: wgpu::Sampler,
}

impl<'window> Renderer<'window> {
    pub async fn new_async(window: Arc<Window>) -> Renderer<'window> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                experimental_features: Default::default(),
                memory_hints: Performance,
                trace: Trace::Off,
            })
            .await
            .expect("Failed to create device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        let sprite_pipeline = SpritePipeline::new(&device, surface_config.format);

        let quad_vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Quad Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * 6) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quad_vertex_count = 6;

        let globals = Globals {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            aspect: surface_config.width as f32 / surface_config.height as f32,
            _padding: [0.0; 7],
        };

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Globals Buffer"),
            contents: bytemuck::bytes_of(&globals),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Pixel Art Sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            sprite_pipeline,
            quad_vertex_buffer,
            quad_vertex_count,
            globals_buffer,
            textures: HashMap::new(),
            nearest_sampler,
        }
    }

    pub fn new(window: Arc<Window>) -> Renderer<'window> {
        pollster::block_on(Renderer::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self, queue: &RenderQueue, camera_matrix: glam::Mat4, virtual_size: Vec2) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = &surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::bytes_of(&Globals {
                view_proj: camera_matrix.to_cols_array_2d(),
                aspect: (virtual_size.x / virtual_size.y)
                    / (self.surface_config.width as f32 / self.surface_config.height as f32),
                _padding: [0.0; 7],
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
                    // Apply Texture Aspect
                    let tex_width = texture.inner.width as f32;
                    let tex_height = texture.inner.height as f32;

                    let world_width = (tex_width / 16.0) * scale.x;
                    let world_height = (tex_height / 16.0) * scale.y;

                    let world_vertices: Vec<Vertex> = [
                        // Треугольник 1
                        Vertex {
                            position: [-world_width * 0.5, -world_height * 0.5],
                            tex_coords: [0.0, 0.0],
                        },
                        Vertex {
                            position: [world_width * 0.5, -world_height * 0.5],
                            tex_coords: [1.0, 0.0],
                        },
                        Vertex {
                            position: [-world_width * 0.5, world_height * 0.5],
                            tex_coords: [0.0, 1.0],
                        },
                        // Треугольник 2
                        Vertex {
                            position: [world_width * 0.5, -world_height * 0.5],
                            tex_coords: [1.0, 0.0],
                        },
                        Vertex {
                            position: [world_width * 0.5, world_height * 0.5],
                            tex_coords: [1.0, 1.0],
                        },
                        Vertex {
                            position: [-world_width * 0.5, world_height * 0.5],
                            tex_coords: [0.0, 1.0],
                        },
                    ]
                    .iter()
                    .map(|v| {
                        // Apply Scale
                        let pos_3d = Vec3::new(v.position[0], v.position[1], 0.0);

                        let scaled = pos_3d * Vec3::new(scale.x, scale.y, 1.0);

                        // Apply Rotation
                        let rotated = rotation * scaled;

                        let final_pos = Vec2::new(position.x + rotated.x, position.y + rotated.y);

                        // Apply final
                        Vertex {
                            position: [final_pos.x, final_pos.y],
                            tex_coords: v.tex_coords,
                        }
                    })
                    .collect();

                    // Обновляем vertex buffer
                    self.queue.write_buffer(
                        &self.quad_vertex_buffer,
                        0,
                        bytemuck::cast_slice(&world_vertices),
                    );

                    let key = Arc::as_ptr(&texture.inner) as usize;

                    let gpu_texture = self.textures.entry(key).or_insert_with(|| {
                        GpuTexture::from_asset(&self.device, &self.queue, &texture.inner)
                    });

                    let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.sprite_pipeline.bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: self.globals_buffer.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&gpu_texture.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&self.nearest_sampler), // ← сэмплер!
                            },
                        ],
                        label: Some("Sprite BindGroup"),
                    });

                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Sprite Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                                depth_slice: None,
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
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
