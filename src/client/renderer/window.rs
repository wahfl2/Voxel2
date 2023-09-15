use wgpu::{SurfaceConfiguration, Surface, Instance, TextureUsages, Device, Adapter};
use winit::dpi::PhysicalSize;

pub struct WindowBuilder {
    pub inner: winit::window::Window,
    pub surface: Surface,
    pub size: PhysicalSize<u32>,
}

pub struct Window {
    pub inner: winit::window::Window,
    pub surface: Surface,
    pub size: PhysicalSize<u32>,
    pub config: SurfaceConfiguration,
}

impl WindowBuilder {
    pub fn new(inner: winit::window::Window, instance: &Instance) -> Self {
        let size = inner.inner_size();
        let surface = unsafe { instance.create_surface(&inner) }.unwrap();
        Self { 
            surface, 
            size, 
            inner,
        }
    }

    pub fn build(self, device: &Device, adapter: &Adapter) -> Window {
        let surface_capabilities = self.surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .expect("Surface not capable of sRGB??");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        self.surface.configure(device, &config);

        Window::new(self, config)
    }
}

impl Window {
    fn new(builder: WindowBuilder, config: SurfaceConfiguration) -> Self {
        Self {
            inner: builder.inner,
            surface: builder.surface,
            size: builder.size,
            config,
        }
    }

    pub fn resize(&mut self, device: &Device, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(device, &self.config);
        }
    }

    pub fn reconfigure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }
}