use std::{collections::HashMap, sync::Arc};

use crate::{
    font::FontManager, pipelines::MeshPipeline, pipelines::SpritePipeline,
    resources::texture::GpuTexture,
};
use glam::Vec2;
use runa_asset::TextureAsset;
use runa_render_api::{RenderCommands, RenderQueue};
use wgpu::util::DeviceExt;
use wgpu::{MemoryHints::Performance, Trace};
use wgpu::{Texture, TextureView};
use winit::window::Window;

/// Per-instance data for sprite/tile rendering.
/// Contains transform, UV coordinates, and flip information.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub position: [f32; 3],  // x, y, z
    pub rotation: f32,       // radians
    pub scale: [f32; 3],     // x, y, z
    pub uv_offset: [f32; 2], // left-bottom UV coordinates
    pub uv_size: [f32; 2],   // UV quad size
    pub flip: u32,           // bit 0 = flip_x, bit 1 = flip_y
    pub _pad: f32,
}

/// Vertex structure for sprite quads.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

/// Global uniform buffer data containing view-projection matrix and aspect ratio.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub view_proj: [[f32; 4]; 4],
    pub aspect: f32,
    pub _padding: [f32; 7],
}

/// Main renderer struct managing GPU resources and rendering.
pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    sprite_pipeline: SpritePipeline,

    mesh_pipeline: MeshPipeline,

    globals_buffer: wgpu::Buffer,

    textures: HashMap<usize, GpuTexture>,
    nearest_sampler: wgpu::Sampler,

    font_manager: FontManager,

    textures_cache: HashMap<usize, Arc<TextureAsset>>,
    bind_group_cache: HashMap<usize, wgpu::BindGroup>,

    depth_view: TextureView,
    depth_texture: Texture,

    /// Base quad vertices (6 vertices, static).
    quad_buffer: wgpu::Buffer,
    /// Dynamic instance buffer - resized as needed.
    instance_buffer: wgpu::Buffer,
    /// Current capacity of instance buffer in number of instances.
    instance_buffer_capacity: usize,
}

impl<'window> Renderer<'window> {
    /// Creates a new renderer with the given window and vsync setting.
    ///
    /// # Arguments
    /// * `window` - The window to render to
    /// * `vsync` - Enable vertical sync for frame presentation
    pub async fn new_async(window: Arc<Window>, vsync: bool) -> Self {
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

        let surface_config: wgpu::SurfaceConfiguration;
        if vsync {
            surface_config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_capabilities(&adapter).formats[0],
                width: size.width.max(1),
                height: size.height.max(1),
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };
        } else {
            surface_config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_capabilities(&adapter).formats[0],
                width: size.width.max(1),
                height: size.height.max(1),
                present_mode: wgpu::PresentMode::AutoNoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };
        }

        surface.configure(&device, &surface_config);

        let sprite_pipeline = SpritePipeline::new(&device, surface_config.format);

        let identity_mat = glam::Mat4::IDENTITY.to_cols_array_2d();
        let globals = Globals {
            view_proj: identity_mat,
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

        let font_manager = FontManager::new(&device, &queue);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"),
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 3D mesh pipeline
        let mesh_pipeline = MeshPipeline::new(
            &device,
            surface_config.format,
            wgpu::TextureFormat::Depth32Float,
        );

        const QUAD_VERTICES: &[Vertex] = &[
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let quad_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Instance buffer with initial capacity
        const INITIAL_INSTANCE_CAPACITY: usize = 1000;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * INITIAL_INSTANCE_CAPACITY) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            surface_config,
            device,
            queue,
            sprite_pipeline,
            mesh_pipeline,
            globals_buffer,
            textures: HashMap::new(),
            nearest_sampler,
            font_manager,
            textures_cache: HashMap::new(),
            bind_group_cache: HashMap::new(),
            depth_view,
            depth_texture,
            quad_buffer,
            instance_buffer,
            instance_buffer_capacity: INITIAL_INSTANCE_CAPACITY,
        }
    }

    /// Creates a new renderer synchronously (blocking).
    pub fn new(window: Arc<Window>, vsync: bool) -> Self {
        pollster::block_on(Self::new_async(window, vsync))
    }

    /// Resizes the surface and recreates the depth texture.
    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);

        // Recreate depth texture
        self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.surface_config.width,
                height: self.surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"),
            view_formats: &[],
        });
        self.depth_view = self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    }

    /// Renders the current frame using the provided render queue and camera matrix.
    ///
    /// # Arguments
    /// * `queue` - The render queue containing draw commands
    /// * `camera_matrix` - View-projection matrix for the camera
    /// * `virtual_size` - Virtual resolution size for aspect ratio calculation
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

        // Update global uniform data
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

        // ===== STEP 1: COLLECT INSTANCES BY TEXTURE =====
        let mut all_instances = Vec::new();
        let mut batches = Vec::new();

        for cmd in &queue.commands {
            match cmd {
                RenderCommands::Sprite {
                    texture,
                    position,
                    rotation,
                    scale,
                } => {
                    let tex_width = texture.width as f32;
                    let tex_height = texture.height as f32;
                    let world_scale_x = scale.x * (tex_width / 16.0);
                    let world_scale_y = scale.y * (tex_height / 16.0);

                    let instance = InstanceData {
                        position: [position.x, position.y, position.z],
                        rotation: rotation.z,
                        scale: [world_scale_x, world_scale_y, 1.0],
                        uv_offset: [0.0, 0.0],
                        uv_size: [1.0, 1.0],
                        flip: 0,
                        _pad: 0.0,
                    };

                    let key = Arc::as_ptr(texture) as usize;
                    if !self.textures_cache.contains_key(&key) {
                        self.textures_cache.insert(key, texture.clone());
                    }

                    let offset = all_instances.len();
                    all_instances.push(instance);
                    batches.push((key, offset, 1));
                }
                RenderCommands::Tile {
                    texture,
                    position,
                    size,
                    uv_rect,
                    flip_x,
                    flip_y,
                    color: _,
                } => {
                    let instance = InstanceData {
                        position: [position.x, position.y, position.z],
                        rotation: 0.0,
                        scale: [size.x as f32, size.y as f32, 1.0],
                        uv_offset: [uv_rect[0], uv_rect[1]],
                        uv_size: [uv_rect[2], uv_rect[3]],
                        flip: ((*flip_x) as u32) | (((*flip_y) as u32) << 1),
                        _pad: 0.0,
                    };

                    let key = Arc::as_ptr(texture) as usize;
                    if !self.textures_cache.contains_key(&key) {
                        self.textures_cache.insert(key, texture.clone());
                    }

                    let offset = all_instances.len();
                    all_instances.push(instance);
                    batches.push((key, offset, 1));
                }
                RenderCommands::DebugRect { .. } => {
                    // Debug rectangles are not yet implemented
                }
                RenderCommands::Text { .. } => {
                    // TODO: implement text rendering
                }
            }
        }

        // Resize instance buffer if needed
        if all_instances.len() > self.instance_buffer_capacity {
            // Grow buffer by 1.5x to reduce reallocations
            let new_capacity = (all_instances.len() * 3 / 2).max(1000);
            self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (std::mem::size_of::<InstanceData>() * new_capacity) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.instance_buffer_capacity = new_capacity;
        }

        // Write all instances to GPU buffer
        if !all_instances.is_empty() {
            self.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&all_instances),
            );
        }

        // ===== STEP 2: RENDER BATCHES =====
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

        // Render each texture batch using instancing
        for (texture_key, instance_offset, instance_count) in batches {
            let gpu_texture = self.textures.entry(texture_key).or_insert_with(|| {
                let texture = self.textures_cache.get(&texture_key).unwrap();
                GpuTexture::from_asset(&self.device, &self.queue, texture)
            });

            let bind_group = self.bind_group_cache.entry(texture_key).or_insert_with(|| {
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    label: Some("BindGroup"),
                })
            });

            rpass.set_pipeline(&self.sprite_pipeline.pipeline);
            rpass.set_bind_group(0, &*bind_group, &[]);
            rpass.set_vertex_buffer(0, self.quad_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            // Draw instanced: 6 vertices per quad, instanced over the batch range
            rpass.draw(
                0..6,
                instance_offset as u32..(instance_offset + instance_count) as u32,
            );
        }

        drop(rpass);
        self.queue.submit(Some(encoder.finish()));
        let _ = self.device.poll(wgpu::PollType::Poll);
        surface_texture.present();
    }
}
