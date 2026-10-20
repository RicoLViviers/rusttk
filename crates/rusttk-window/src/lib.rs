use std::error::Error;

use winit::{
    application::ApplicationHandler, 
    event::WindowEvent, 
    event_loop::{
        ActiveEventLoop, 
        EventLoop
    }, 
    window::{
        Window, 
        WindowId
    }
};

struct WindowApp {
    window: Option<Window>,
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Test Window");
        self.window = Some(event_loop.create_window(window_attributes).unwrap())
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("{event:?}");

        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                window.request_redraw();
            },
            _ => (),
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let mut app = WindowApp { window: None };

    event_loop.run_app(&mut app)?;

    Ok(())
}