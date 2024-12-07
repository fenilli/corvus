use std::collections::HashMap;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(PartialEq, Eq, Debug)]
pub enum InputState {
    Idle,
    Pressed,
    Held,
    Released,
}

pub struct Input {
    key_states: HashMap<KeyCode, InputState>,
    mouse_states: HashMap<MouseButton, InputState>,
    cursor_position: PhysicalPosition<f64>,
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            mouse_states: HashMap::new(),
            cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }

    fn keyboard_input(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };

        let input_state = self.key_states.entry(key_code).or_insert(InputState::Idle);
        match event.state {
            ElementState::Pressed => {
                if *input_state != InputState::Held {
                    *input_state = InputState::Pressed
                }
            }
            ElementState::Released => *input_state = InputState::Released,
        };
    }

    fn mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
        let input_state = self.mouse_states.entry(*button).or_insert(InputState::Idle);
        match state {
            ElementState::Pressed => {
                if *input_state != InputState::Held {
                    *input_state = InputState::Pressed
                }
            }
            ElementState::Released => *input_state = InputState::Released,
        };
    }

    fn cursor_moved(&mut self, position: &PhysicalPosition<f64>) {
        self.cursor_position = *position;
    }

    fn contains_key_with_value<K, V>(map: &HashMap<K, V>, key: &K, value: &V) -> bool
    where
        K: Eq + std::hash::Hash,
        V: PartialEq,
    {
        map.get(key) == Some(value)
    }

    pub fn start_step(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.keyboard_input(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_input(state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_moved(position);
            }
            _ => (),
        }
    }

    pub fn end_step(&mut self) {
        for input_state in self.key_states.values_mut() {
            if *input_state == InputState::Pressed {
                *input_state = InputState::Held
            } else if *input_state == InputState::Released {
                *input_state = InputState::Idle
            }
        }
    }

    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        Input::contains_key_with_value(&self.key_states, &key_code, &InputState::Pressed)
    }

    pub fn key_released(&self, key_code: KeyCode) -> bool {
        Input::contains_key_with_value(&self.key_states, &key_code, &InputState::Released)
    }

    pub fn key_held(&self, key_code: KeyCode) -> bool {
        Input::contains_key_with_value(&self.key_states, &key_code, &InputState::Held)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        Input::contains_key_with_value(&self.mouse_states, &button, &InputState::Pressed)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        Input::contains_key_with_value(&self.mouse_states, &button, &InputState::Released)
    }

    pub fn mouse_held(&self, button: MouseButton) -> bool {
        Input::contains_key_with_value(&self.mouse_states, &button, &InputState::Held)
    }

    pub fn cursor_position(&self) -> PhysicalPosition<f64> {
        self.cursor_position
    }
}
