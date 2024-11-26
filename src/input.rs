use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
};

pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn keyboard_input(&mut self, event: KeyEvent) {
        println!("keyboard {:?}", event);
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        println!("mouse {:?} {:?}", state, button);
    }

    pub fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
        println!("delta {:?}", delta);
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        println!("cursor moved {:?}", position);
    }
}
