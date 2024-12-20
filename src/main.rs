use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event_loop::EventLoop, window::Window,
};

use corvus::App;

struct AppDescriptor {
    pub title: &'static str,
    pub size: PhysicalSize<u32>,
}

enum AppState {
    Initializing(AppDescriptor),
    Running(App),
}

struct WinitApp {
    app: AppState,
}

impl WinitApp {
    pub fn new(descriptor: AppDescriptor) -> Self {
        Self {
            app: AppState::Initializing(descriptor),
        }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let AppState::Initializing(ref descriptor) = self.app else {
            return;
        };

        let window_attributes = Window::default_attributes()
            .with_title(descriptor.title)
            .with_inner_size(descriptor.size);
        let Ok(window) = event_loop.create_window(window_attributes) else {
            return;
        };

        self.app = AppState::Running(App::new(window));
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

        if let AppState::Running(app) = &mut self.app {
            app.window_event(event, event_loop);
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = WinitApp::new(AppDescriptor {
        title: "Corvus",
        size: PhysicalSize::new(800, 600),
    });
    _ = event_loop.run_app(&mut app);
}
