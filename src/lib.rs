use core::panic;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{self, Window},
};

use state::State;

mod state;
mod math;

pub async fn run() {
    let mut app_handler = AppHandler::default();
    let event_loop = EventLoop::new().unwrap();

    let result = event_loop.run_app(&mut app_handler);

    if result.is_err() {
        panic!("Cannot run application: {}", result.unwrap_err());
    }

    let state = State::new(app_handler.window.as_ref().unwrap()).await;
}


#[derive(Default)]
struct AppHandler {
    window: Option<Window>,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if window_id != self.window.as_ref().unwrap().id() {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
