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
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
    pub aspect: f32,
    pub _padding: [f32; 7],
}

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    sprite_pipeline: SpritePipeline,

    vertex_buffer: wgpu::Buffer,
    max_vertices: usize,

    globals_buffer: wgpu::Buffer,

    textures: HashMap<usize, GpuTexture>,
    nearest_sampler: wgpu::Sampler,
}

impl<'window> Renderer<'window> {
    pub async fn new_async(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
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
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width.max(1),
            height: size.height.max(1),
            // ← КЛЮЧЕВОЕ ИЗМЕНЕНИЕ:
            present_mode: wgpu::PresentMode::Immediate, // вместо Fifo
            alpha_mode: surface.get_capabilities(&adapter).alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let sprite_pipeline = SpritePipeline::new(&device, surface_config.format);

        const MAX_SPRITES: usize = 1000;
        const VERTICES_PER_SPRITE: usize = 6;
        let max_vertices = MAX_SPRITES * VERTICES_PER_SPRITE;
        let vertex_buffer_size = (std::mem::size_of::<Vertex>() * max_vertices) as u64;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: vertex_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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
            device,
            queue,
            sprite_pipeline,
            vertex_buffer,
            max_vertices,
            globals_buffer,
            textures: HashMap::new(),
            nearest_sampler,
        }
    }

    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self, queue: &RenderQueue, camera_matrix: glam::Mat4, virtual_size: Vec2) {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(tex) => tex,
            Err(_) => return,
        };

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: None,
                ..Default::default()
            });

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

        let mut all_vertices = Vec::with_capacity(queue.commands.len() * 6);
        for cmd in &queue.commands {
            match cmd {
                RenderCommands::Sprite {
                    texture,
                    position,
                    rotation,
                    scale,
                } => {
                    let tex_width = texture.inner.width as f32;
                    let tex_height = texture.inner.height as f32;

                    let world_width = (tex_width / 16.0) * scale.x;
                    let world_height = (tex_height / 16.0) * scale.y;

                    let sprite_vertices: [Vertex; 6] = [
                        Vertex {
                            position: [-world_width * 0.5, -world_height * 0.5],
                            tex_coords: [0.0, 1.0],
                        },
                        Vertex {
                            position: [world_width * 0.5, -world_height * 0.5],
                            tex_coords: [1.0, 1.0],
                        },
                        Vertex {
                            position: [-world_width * 0.5, world_height * 0.5],
                            tex_coords: [0.0, 0.0],
                        },
                        Vertex {
                            position: [world_width * 0.5, -world_height * 0.5],
                            tex_coords: [1.0, 1.0],
                        },
                        Vertex {
                            position: [world_width * 0.5, world_height * 0.5],
                            tex_coords: [1.0, 0.0],
                        },
                        Vertex {
                            position: [-world_width * 0.5, world_height * 0.5],
                            tex_coords: [0.0, 0.0],
                        },
                    ];

                    let transformed_vertices: Vec<Vertex> = sprite_vertices
                        .iter()
                        .map(|v| {
                            let pos_3d = Vec3::new(v.position[0], v.position[1], 0.0);
                            let scaled = pos_3d * Vec3::new(scale.x, scale.y, 1.0);
                            let rotated = rotation * scaled;
                            let final_pos =
                                Vec2::new(position.x + rotated.x, position.y + rotated.y);

                            Vertex {
                                position: [final_pos.x, final_pos.y],
                                tex_coords: v.tex_coords,
                            }
                        })
                        .collect();

                    all_vertices.extend(transformed_vertices);
                }
            }
        }

        if all_vertices.len() > self.max_vertices {
            eprintln!(
                "Too many vertices! Max: {}, Current: {}",
                self.max_vertices,
                all_vertices.len()
            );
            return;
        }

        if !all_vertices.is_empty() {
            self.queue
                .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&all_vertices));
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Sprite Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        let mut vertex_offset = 0;
        for cmd in &queue.commands {
            match cmd {
                RenderCommands::Sprite { texture, .. } => {
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
                                resource: wgpu::BindingResource::Sampler(&self.nearest_sampler),
                            },
                        ],
                        label: Some("Sprite BindGroup"),
                    });

                    rpass.set_pipeline(&self.sprite_pipeline.pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_vertex_buffer(0, self.vertex_buffer.slice(vertex_offset..));
                    rpass.draw(0..6, 0..1);

                    vertex_offset += (std::mem::size_of::<Vertex>() * 6) as u64;
                }
            }
        }

        drop(rpass);
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
