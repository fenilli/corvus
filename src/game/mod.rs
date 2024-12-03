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
        world.register::<Player>();
        // world.register_component::<Enemy>();

        let player = world.spawn();
        world.insert(player, Player);

        world.spawn();
        world.spawn();

        world.despawn(player);

        let enemy = world.spawn();
        world.insert(enemy, Player);

        world.spawn();

        println!("@Init -> {:?}", world);

        for (entity, player) in world
            .entities()
            .zip(world.components::<Player>().unwrap())
            .filter_map(|(entity, player)| Some((entity, player?)))
        {
            println!("@For -> {:?} {:?}", entity, player);
        }

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            world,
        }
    }

    pub fn update(&mut self) {
        for _delta_time in self.clock.update() {
            // if let Some(iter) = self.world.iter_components::<Player>() {
            //     for (entity, player) in iter {
            //         println!("@Entity: {:?}", entity);
            //     }
            // }
        }

        // println!("@Update -> {:?}", self.world);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
