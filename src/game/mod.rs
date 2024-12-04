mod components;
mod systems;

use std::thread::sleep;

use image::GenericImageView;
use winit::window::Window;

use crate::{
    renderer::Renderer,
    resources::{AssetLoader, Clock, Input},
    world::World,
};

pub struct Game {
    input: Input,
    clock: Clock,
    renderer: Renderer,
    asset_loader: AssetLoader,

    world: World,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut asset_loader = AssetLoader::new();
        let mut world = World::new();

        let image_handle = asset_loader.load_image("assets/black_square.png").unwrap();
        println!("{:?}", image_handle);
        let image = asset_loader.get_image(image_handle).unwrap();
        println!("{:?}", image.dimensions());

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            asset_loader,

            world,
        }
    }

    pub fn update(&mut self) {
        for _delta_time in self.clock.update() {
            // println!("@Update -> {:?}", self.world);
        }

        sleep(self.clock.frame_duration());
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
