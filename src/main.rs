use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event_loop::EventLoop, window::Window,
};

use corvus::Game;

struct WinitApp {
    game: Option<Game>,
}

impl WinitApp {
    pub fn new() -> Self {
        Self { game: None }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Corvus")
            .with_inner_size(PhysicalSize::new(1280, 720));

        let window = event_loop.create_window(window_attributes).unwrap();

        self.game = Some(Game::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event_loop.exiting() {
            return;
        }

        if let Some(game) = &mut self.game {
            game.process_window_event(event_loop, event);
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    _ = event_loop.run_app(&mut WinitApp::new());
}
