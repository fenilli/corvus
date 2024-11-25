mod ecs;
mod game;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

use game::states::app_state::{self, AppState};

enum LifecycleState {
    Initializing,
    Running(AppState),
    Closing,
}

struct App {
    lifecycle_state: LifecycleState,
}

impl App {
    pub fn new() -> Self {
        Self {
            lifecycle_state: LifecycleState::Initializing,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self.lifecycle_state {
            LifecycleState::Initializing => {
                let app_state = AppState::new(
                    event_loop
                        .create_window(Window::default_attributes().with_title("Corvus"))
                        .unwrap(),
                );

                self.lifecycle_state = LifecycleState::Running(app_state);
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
        match &mut self.lifecycle_state {
            LifecycleState::Running(app_state) => match event {
                winit::event::WindowEvent::CloseRequested => {
                    self.lifecycle_state = LifecycleState::Closing
                }
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
                        self.lifecycle_state = LifecycleState::Closing;
                    }
                    _ => (),
                },
                winit::event::WindowEvent::RedrawRequested => {
                    app_state.update();
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &self.lifecycle_state {
            LifecycleState::Running(app_state) => app_state.window().request_redraw(),
            LifecycleState::Closing => event_loop.exit(),
            _ => (),
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    _ = event_loop.run_app(&mut app);
}
