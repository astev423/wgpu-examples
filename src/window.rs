use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::PhysicalKey,
};

use crate::gpu_pipeline::State;

const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

pub struct Window {
    graphics: Option<State>,
    next_frame_time: Instant,
    window: Option<Arc<winit::window::Window>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            graphics: None,
            next_frame_time: Instant::now(),
            window: None,
        }
    }
}

impl ApplicationHandler<State> for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed");
        let window = Arc::new(
            event_loop
                .create_window(winit::window::Window::default_attributes())
                .unwrap(),
        );

        self.window = Some(window.clone());
        self.graphics = Some(pollster::block_on(State::new(window.clone())).unwrap());

        self.next_frame_time = Instant::now();
        window.request_redraw();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.graphics = Some(event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();

        if now >= self.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        } else {
            // This WAITS until given time, then WAKES UP, then after waking it sees it has
            // nothing to do so then goes to WAIT again, but when it waits the next frame time
            // will be less than now so it triggers rerender
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(graphics) = self.graphics.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                graphics.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                graphics.update();

                if let Err(e) = graphics.render() {
                    log::error!("{e}");
                    event_loop.exit();
                    return;
                }

                self.next_frame_time = Instant::now() + FRAME_DURATION;
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
            } => {
                graphics.handle_key(event_loop, code, key_state.is_pressed());
            }
            _ => {}
        }
    }
}
