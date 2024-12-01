// mod components;
// mod systems;

// use crate::world::JoinIterators;
// use components::Quad;
// use components::{Position, Rotation, Scale, Transform, Velocity};
// use systems::quad_system;
use winit::window::Window;

use crate::{
    renderer::Renderer,
    resources::{Clock, Input},
    world::World,
};

// fn player_input_system(world: &World, input: &Input) {
//     let player_speed = 100.0;

//     let players = world.iter_components::<Player>();
//     let velocities = world.iter_components_mut::<Velocity>();
//     let transforms = world.iter_components_mut::<Transform>();

//     let Some(iter) = players.join(velocities).join(transforms) else {
//         return;
//     };

//     for (entity, ((player, mut velocity), mut transform)) in iter {
//         println!("{:?} {:?}", entity, player);
//         if input.key_held(winit::keyboard::KeyCode::KeyA) {
//             velocity.y = -player_speed;
//         }

//         if input.key_held(winit::keyboard::KeyCode::KeyD) {
//             velocity.y = player_speed;
//         }
//     }
// }

// fn movement_system(world: &World, delta_time: f32) {
//     let velocities = world.iter_components::<Velocity>();
//     let transforms = world.iter_components_mut::<Transform>();

//     let Some(iter) = velocities.join(transforms) else {
//         return;
//     };

//     for (entity, (velocity, mut transform)) in iter {
//         transform.position.y += velocity.y * delta_time;

//         println!(
//             "for entity: {:?} -> x: {} y: {}",
//             entity, transform.position.x, transform.position.y
//         );
//     }
// }

#[derive(Debug)]
struct Player;
// impl Component for Player {}

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
        let player = world.create_entity();
        world.set_component(player, Player);

        println!("{:#?}", world);
        // world.register_component::<Player>();
        // world.register_component::<Transform>();
        // world.register_component::<Velocity>();

        // let player = world.create_entity();
        // world.set_component(player, Player);
        // world.set_component(
        //     player,
        //     Transform::new(
        //         Position::new(100.0, 100.0),
        //         Rotation::new(0.0),
        //         Scale::new(0.0),
        //     ),
        // );
        // world.set_component(player, Velocity::new(0.0, 0.0));

        // let enemy = world.create_entity();
        // world.set_component(
        //     enemy,
        //     Transform::new(
        //         Position::new(100.0, 100.0),
        //         Rotation::new(0.0),
        //         Scale::new(0.0),
        //     ),
        // );
        // world.set_component(enemy, Velocity::new(0.0, 0.0));

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            world,
        }
    }

    pub fn update(&mut self) {
        // quad_system(&self.world, &mut self.renderer);

        for _delta_time in self.clock.update() {
            //     player_input_system(&self.world, &self.input);
            //     movement_system(&self.world, delta_time);
        }

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
