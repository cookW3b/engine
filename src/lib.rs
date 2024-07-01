use core::panic;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{self, Window},
};

use std::sync::Arc;

use state::State;

mod math;
mod state;
mod camera;
mod camera_cgmath;

pub async fn run() {
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();

    let result = event_loop.run_app(&mut app);

    if result.is_err() {
        panic!("Cannot run application: {}", result.unwrap_err());
    }
}

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
            self.window = Some(window.clone());
            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
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
            WindowEvent::Resized(size) => {
                if self.state.is_some() {
                    self.state.as_mut().unwrap().resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
                if self.state.is_some() {
                    match self.state.as_ref().unwrap().render() {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e)
                    }
                }
            }
            _ => (),
        }
    }
}
