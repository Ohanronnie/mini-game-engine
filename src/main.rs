use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{EventLoop}, window::Window};

#[derive(Default)]
struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Caeno")).expect("failed to create window");

        self.window = Some(window)
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => ()
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().expect("failed to create a new event loop");
    let mut application = App::default();
    event_loop.run_app(&mut application).expect("failed to run app");
}
