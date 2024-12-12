use winit::keyboard::KeyCode;

use crate::{
    app::{context::AppContext, scene::Scene},
    ecs::{
        components::{Camera, Rectangle, Sprite, Transform},
        Entity,
    },
};

pub struct Game {
    player_slots: [Entity; 4],
    enemy_slots: [Entity; 4],
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_slots: [
                Entity::DANGLING,
                Entity::DANGLING,
                Entity::DANGLING,
                Entity::DANGLING,
            ],
            enemy_slots: [
                Entity::DANGLING,
                Entity::DANGLING,
                Entity::DANGLING,
                Entity::DANGLING,
            ],
        }
    }
}

impl Scene for Game {
    fn enter(&mut self, context: &mut AppContext) {
        context.world.register::<Sprite>();
        context.world.register::<Transform>();
        context.world.register::<Rectangle>();

        let window_size = context.system_configuration.window_size;
        let player_slots = self.player_slots.len();

        let camera = context.world.spawn();
        context
            .world
            .insert(camera, Camera::new(window_size.width, window_size.height));

        for slot_id in 0..player_slots {
            let entity = context.world.spawn();
            context.world.insert(entity, Sprite);
            context.world.insert(
                entity,
                Transform::from_xy(
                    50. + (75. * slot_id as f32),
                    (window_size.width / 2) as f32 - (100. / 2.),
                ),
            );
            context.world.insert(entity, Rectangle::new(50, 100));

            self.player_slots[slot_id] = entity;
        }
    }

    fn fixed_update(&mut self, _delta_time: f32, context: &mut AppContext) {
        if context.input.key_held(KeyCode::KeyA) {
            for entity in self.player_slots {
                let Some(mut transform) = context.world.get_component_mut::<Transform>(entity)
                else {
                    continue;
                };

                transform.position.x -= 10.;
            }
        } else if context.input.key_held(KeyCode::KeyD) {
            for entity in self.player_slots {
                let Some(mut transform) = context.world.get_component_mut::<Transform>(entity)
                else {
                    continue;
                };

                transform.position.x += 10.;
            }
        }
    }

    fn update(&mut self, _delta_time: f32, _context: &mut AppContext) {}

    fn exit(&mut self, _context: &mut AppContext) {}
}
