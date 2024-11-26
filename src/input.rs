use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(PartialEq, Eq)]
pub enum InputAction<T> {
    Held(T),
    Pressed(T),
    Released(T),
}

pub struct Input {
    key_actions: Vec<InputAction<KeyCode>>,
    mouse_actions: Vec<InputAction<MouseButton>>,
    cursor_position: PhysicalPosition<f64>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_actions: Vec::new(),
            mouse_actions: Vec::new(),
            cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        self.key_actions.clear();
        self.mouse_actions.clear();
    }

    pub fn keyboard_input(&mut self, event: KeyEvent) {
        match event.state {
            ElementState::Pressed => {
                let PhysicalKey::Code(key_code) = event.physical_key else {
                    return;
                };

                if event.repeat {
                    self.key_actions.push(InputAction::Held(key_code));
                } else {
                    self.key_actions.push(InputAction::Pressed(key_code));
                }
            }
            ElementState::Released => {
                let PhysicalKey::Code(key_code) = event.physical_key else {
                    return;
                };

                self.key_actions.push(InputAction::Released(key_code));
            }
        };
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                self.mouse_actions.push(InputAction::Pressed(button));
            }
            ElementState::Released => {
                self.mouse_actions.push(InputAction::Released(button));
            }
        };
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = position;
    }

    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        self.key_actions.contains(&InputAction::Pressed(key_code))
    }

    pub fn key_released(&self, key_code: KeyCode) -> bool {
        self.key_actions.contains(&InputAction::Released(key_code))
    }

    pub fn key_held(&self, key_code: KeyCode) -> bool {
        self.key_actions.contains(&InputAction::Held(key_code))
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_actions.contains(&InputAction::Pressed(button))
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_actions.contains(&InputAction::Released(button))
    }

    pub fn cursor_position(&self) -> PhysicalPosition<f64> {
        self.cursor_position
    }
}
