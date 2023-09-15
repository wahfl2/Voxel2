use wgpu::{Device, Queue, Instance, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference, Features, Limits, DeviceDescriptor, SurfaceError, TextureViewDescriptor, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, CommandEncoderDescriptor, CommandEncoder, TextureView, Color};
use winit::dpi::PhysicalSize;

use self::{window::{Window, WindowBuilder}, shaders::Shaders, pipeline::Pipeline};

mod window;
mod shaders;
mod pipeline;

pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    pub window: Window,
    pub shaders: Shaders,
    pub pipeline: Pipeline,
}

impl Renderer {
    const BACKENDS: Backends = Backends::VULKAN.union(Backends::METAL);

    pub async fn new(inner_window: winit::window::Window) -> Self {
        let instance = Instance::new(InstanceDescriptor {
            backends: Self::BACKENDS,
            ..Default::default()
        });
        
        let builder = WindowBuilder::new(inner_window, &instance);

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&builder.surface),
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

        let window = builder.build(&device, &adapter);
        let shaders = Shaders::new(&device);
        let pipeline = Pipeline::new(&device, &shaders, window.config.format);

        Self {
            device,
            queue,
            window,
            shaders,
            pipeline,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.window.resize(&self.device, new_size);
    }

    pub fn reconfigure(&self) {
        self.window.reconfigure(&self.device);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor::default());

        self.initial_pass(&mut encoder, &view);

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }

    fn initial_pass(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Initial Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::WHITE),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline.inner);
        render_pass.draw(0..3, 0..1);
    }
}