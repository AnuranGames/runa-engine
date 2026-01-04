use winit::window::Window;

pub struct GpuContext<'window> {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'window>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
}

impl<'window> GpuContext<'window> {
    pub async fn new(window: &'window Window) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Runa Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_format = surface.get_capabilities(&adapter).formats[0];

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_format,
        }
    }
}
