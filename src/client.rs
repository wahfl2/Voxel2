use wgpu::SurfaceError;
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}};

use self::renderer::Renderer;

mod renderer;

pub struct Voxel2Client {
    pub event_loop: EventLoop<()>,
    pub renderer: Renderer,
}

impl Voxel2Client {
    pub async fn new() -> Self {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = Renderer::new(window).await;

        Self { event_loop, renderer }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.renderer.window.inner.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    self.renderer.resize(*physical_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.renderer.resize(**new_inner_size);
                }

                _ => {}
            },

            Event::RedrawRequested(window_id) if window_id == self.renderer.window.inner.id() => {
                let render_result = self.renderer.render();

                match render_result {
                    Ok(_) => (),
                    Err(SurfaceError::Lost) => self.renderer.reconfigure(),
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },

            Event::MainEventsCleared => {
                self.renderer.window.inner.request_redraw();
            }

            _ => {}
        });
    }
}