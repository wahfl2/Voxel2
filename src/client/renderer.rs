use wgpu::{Device, Queue, Instance, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference, Features, Limits, DeviceDescriptor};
use winit::dpi::PhysicalSize;

use self::window_surface::Window;

mod window_surface;

pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    pub window: Window,
}

impl Renderer {
    pub async fn new(inner_window: winit::window::Window) -> Self {
        const BACKENDS: Backends = Backends::VULKAN.union(Backends::METAL).union(Backends::DX12);

        let instance = Instance::new(InstanceDescriptor {
            backends: BACKENDS,
            ..Default::default()
        });
        
        let mut window = Window::new(inner_window, &instance);

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&window.surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor { 
                features: Features::empty(), 
                limits: Limits::default(), 
                label: None, 
            }, 
            None
        ).await.unwrap();

        let surface_capabilities = window.surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .expect("Surface not capable of sRGB??");

        window.configure(&device, surface_format, surface_capabilities.alpha_modes[0]);

        Self {
            device,
            queue,
            window,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.window.resize(&self.device, new_size);
    }
}