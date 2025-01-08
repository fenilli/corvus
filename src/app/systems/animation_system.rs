use crate::{
    app::components::{AnimationSet, AnimationState, Sprite},
    ecs::World,
};

pub struct AnimationSystem;

impl AnimationSystem {
    pub fn run_animations(world: &World, interpolated_alpha: f32) {
        for (animation_set, mut sprite, mut animation_state) in
            world.entities().flat_map(|entity| {
                match (
                    world.get_component::<AnimationSet>(entity),
                    world.get_component_mut::<Sprite>(entity),
                    world.get_component_mut::<AnimationState>(entity),
                ) {
                    (Some(animation_set), Some(sprite), Some(animation_state)) => {
                        Some((animation_set, sprite, animation_state))
                    }
                    _ => None,
                }
            })
        {
            let Some(animation) = animation_set.get_animation(&animation_state.current_animation)
            else {
                continue;
            };

            let Some(frame) = animation.get_frame(animation_state.current_frame) else {
                continue;
            };

            animation_state.time += interpolated_alpha;

            if animation_state.time >= frame.duration {
                animation_state.time -= frame.duration;
                animation_state.current_frame += 1;

                if animation_state.current_frame >= animation.frames.len() {
                    animation_state.current_frame = if animation.looping {
                        0
                    } else {
                        animation.frames.len() - 1
                    };
                }
            }

            sprite.region_id = frame.region_id;
        }
    }
}
