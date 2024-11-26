use crate::{ecs::ECS, game::components::TransformComponent};

pub fn movement_system(world: &ECS, key_strength: i32) {
    let transforms = world.get_components_mut::<TransformComponent>();

    if let Some(mut transforms) = transforms {
        for transform in transforms
            .iter_mut()
            .filter_map(|transform| Some(transform))
        {
            transform.position.1 += key_strength
        }
    }
}
