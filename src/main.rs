use std::sync::Arc;

use wgpu::SurfaceStatus::Lost;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window,
};

use crate::renderer::Renderer;
mod renderer;
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Caeno"))
                .expect("failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        self.window = Some(window.clone());
        self.renderer = Some(renderer);
        window.request_redraw();
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(()) => (),
                        Err(_) => {
                            eprintln!("error occurred")
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().expect("failed to create a new event loop");
    let mut application = App::default();
    event_loop
        .run_app(&mut application)
        .expect("failed to run app");
}
