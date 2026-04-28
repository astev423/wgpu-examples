use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::*, event_loop::ActiveEventLoop, keyboard::PhysicalKey,
};

use crate::gpu_pipeline::State;

pub struct Window {
    graphics: Option<State>,
}

impl Window {
    pub fn new() -> Self {
        Self { graphics: None }
    }
}

impl ApplicationHandler<State> for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed window");
        #[allow(unused_mut)]
        let mut window_attributes = winit::window::Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.graphics = Some(pollster::block_on(State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.graphics = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(ref mut graphics) = self.graphics else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => graphics.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                graphics.update();
                println!("Rerendering window and graphics");
                match graphics.render() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => graphics.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}
