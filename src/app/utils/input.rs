#[derive(PartialEq, Eq, Debug)]
enum InputState {
    Idle,
    Pressed,
    Held,
    Released,
}

pub struct Input {
    key_states: std::collections::HashMap<winit::keyboard::KeyCode, InputState>,
    mouse_states: std::collections::HashMap<winit::event::MouseButton, InputState>,
    pub cursor_position: winit::dpi::PhysicalPosition<f64>,
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Self {
            key_states: std::collections::HashMap::new(),
            mouse_states: std::collections::HashMap::new(),
            cursor_position: winit::dpi::PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn process_keyboard_input(&mut self, event: &winit::event::KeyEvent) {
        let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };

        let input_state = self.key_states.entry(key_code).or_insert(InputState::Idle);
        match event.state {
            winit::event::ElementState::Pressed => {
                if *input_state != InputState::Held {
                    *input_state = InputState::Pressed
                }
            }
            winit::event::ElementState::Released => *input_state = InputState::Released,
        };
    }

    pub fn process_mouse_input(
        &mut self,
        state: &winit::event::ElementState,
        button: &winit::event::MouseButton,
    ) {
        let input_state = self.mouse_states.entry(*button).or_insert(InputState::Idle);
        match state {
            winit::event::ElementState::Pressed => {
                if *input_state != InputState::Held {
                    *input_state = InputState::Pressed
                }
            }
            winit::event::ElementState::Released => *input_state = InputState::Released,
        };
    }

    pub fn process_cursor_position(&mut self, position: &winit::dpi::PhysicalPosition<f64>) {
        self.cursor_position = *position;
    }

    pub fn process_end_frame(&mut self) {
        for input_state in self.key_states.values_mut() {
            if *input_state == InputState::Pressed {
                *input_state = InputState::Held
            } else if *input_state == InputState::Released {
                *input_state = InputState::Idle
            }
        }
    }
}
