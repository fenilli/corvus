use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

mod ecs;

enum AppState {
    Initializing,
    Running(Window),
    AboutToClose,
}

struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Initializing,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self.state {
            AppState::Initializing => {
                self.state = AppState::Running(
                    event_loop
                        .create_window(Window::default_attributes().with_title("Corvus"))
                        .unwrap(),
                );
            }
            _ => (),
        };
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &self.state {
            AppState::Running(window) => match event {
                winit::event::WindowEvent::CloseRequested => self.state = AppState::AboutToClose,
                winit::event::WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match logical_key.as_ref() {
                    Key::Named(NamedKey::Escape) => {
                        self.state = AppState::AboutToClose;
                    }
                    _ => (),
                },
                winit::event::WindowEvent::RedrawRequested => {
                    println!("Redraw Requested");
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &self.state {
            AppState::Running(window) => window.request_redraw(),
            AppState::AboutToClose => event_loop.exit(),
            _ => (),
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    _ = event_loop.run_app(&mut app);
}
