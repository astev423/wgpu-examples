use winit::event_loop::EventLoop;

use crate::window::Window;

pub fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut window = Window::new();
    event_loop.run_app(&mut window)?;

    Ok(())
}
