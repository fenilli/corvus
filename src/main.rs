mod game;
mod input;
mod world;

use std::{thread::sleep, time::Duration};

use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

use game::Game;

pub struct AppDescriptor {
    title: &'static str,
    size: PhysicalSize<u32>,
}

enum AppState {
    Initializing(AppDescriptor),
    Running(Game),
    Closing,
}

struct App {
    state: AppState,
}

impl App {
    pub fn new(app_descriptor: AppDescriptor) -> Self {
        Self {
            state: AppState::Initializing(app_descriptor),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let AppState::Initializing(ref app_descriptor) = self.state else {
            return;
        };

        let window_attributes = Window::default_attributes()
            .with_title(app_descriptor.title)
            .with_inner_size(app_descriptor.size);
        let Ok(window) = event_loop.create_window(window_attributes) else {
            return;
        };

        self.state = AppState::Running(Game::new(window));
    }

    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let AppState::Running(app_state) = &mut self.state else {
            return;
        };

        app_state.input().start_step(&event);

        match event {
            WindowEvent::RedrawRequested => {
                app_state.update();
                sleep(Duration::from_secs(1 / 60)); // Simulation of 60 fps.
            }
            WindowEvent::CloseRequested => self.state = AppState::Closing,
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &mut self.state {
            AppState::Running(app_state) => {
                app_state.input().end_step();
                app_state.window().request_redraw();
            }
            AppState::Closing => event_loop.exit(),
            _ => (),
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(AppDescriptor {
        title: "Corvus",
        size: PhysicalSize::new(800, 600),
    });
    _ = event_loop.run_app(&mut app);
}
