use winit::{event::WindowEvent, window::Window};

use super::{
    input::Input,
    scene::{Scene, SceneManager},
    timestep::Timestep,
};
use crate::World;

pub struct Menu;
impl Scene for Menu {
    fn enter(&mut self) {
        println!("enter");
    }

    fn fixed_update(&mut self, delta_time: f32) {
        println!("fixed_update {}", delta_time);
    }

    fn update(&mut self, delta_time: f32) {
        // println!("update {}", delta_time);
    }

    fn exit(&mut self) {
        println!("exit");
    }
}

#[allow(dead_code)]
pub struct App {
    input: Input,
    timestep: Timestep,
    world: World,
    scene_manager: SceneManager,

    window: Window,
}

impl App {
    pub fn new(window: Window) -> Self {
        Self {
            input: Input::new(),
            timestep: Timestep::new(60),
            world: World::new(),
            scene_manager: SceneManager::new(Menu),

            window,
        }
    }

    pub fn window_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.input.start_step(&event);

        match event {
            WindowEvent::RedrawRequested => {
                let (fixed_deltas, variable_delta) = self.timestep.update();

                for delta_time in fixed_deltas {
                    self.scene_manager.fixed_update(delta_time);
                }

                self.scene_manager.update(variable_delta);
                self.input.end_step();
            }
            WindowEvent::CloseRequested => return false,
            _ => (),
        };

        true
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
