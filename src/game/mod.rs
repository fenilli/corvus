use winit::window::Window;

use crate::{
    renderer::Renderer,
    resources::{Clock, Input},
    world::{CommandBuffer, World},
};

#[derive(Debug)]
struct Player;

#[derive(Debug)]
struct Enemy;

pub struct Game {
    input: Input,
    clock: Clock,
    renderer: Renderer,
    world: World,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut world = World::new();
        world.register_component::<Player>();
        world.register_component::<Enemy>();

        let player = world.create_entity();
        world.set_component(player, Player);

        println!("@Init -> {:?}", world);

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            world,
        }
    }

    pub fn update(&mut self) {
        for _delta_time in self.clock.update() {
            // let mut command_buffer = CommandBuffer::new();

            // command_buffer.schedule(|world| {
            //     let enemy = world.create_entity();
            //     world.set_component(enemy, Enemy);
            // });
            // command_buffer.execute(&mut self.world);
        }

        println!("@Update -> {:?}", self.world);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
