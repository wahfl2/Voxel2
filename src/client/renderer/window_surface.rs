use wgpu::{SurfaceConfiguration, Surface, Instance, TextureFormat, CompositeAlphaMode, TextureUsages, Device};
use winit::{dpi::PhysicalSize};

pub struct Window {
    pub inner: winit::window::Window,
    pub surface: Surface,
    pub size: PhysicalSize<u32>,
    pub config: Option<SurfaceConfiguration>,
}

impl Window {
    pub fn new(inner: winit::window::Window, instance: &Instance) -> Self {
        let size = inner.inner_size();
        let surface = unsafe { instance.create_surface(&inner) }.unwrap();
        Self { surface, size, inner, config: None }
    }

    pub fn configure(&mut self, device: &Device, format: TextureFormat, alpha_mode: CompositeAlphaMode) {
        self.config = Some(SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode,
            view_formats: vec![],
        });

        self.surface.configure(device, self.config.as_ref().unwrap())
    }

    pub fn resize(&mut self, device: &Device, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let config = self.config.as_mut().expect("Window not configured!");

            self.size = new_size;
            config.width = new_size.width;
            config.height = new_size.height;
            self.surface.configure(device, config);
        }
    }
}