use crate::{
    ecs::components::{Sprite, Transform},
    render::Renderer,
    World,
};

pub fn render_system(world: &mut World, renderer: &mut Renderer) {
    for transform in world.entities().filter_map(|entity| {
        let (Some(_), Some(transform)) = (
            world.get_component::<Sprite>(entity),
            world.get_component::<Transform>(entity),
        ) else {
            return None;
        };

        Some(transform)
    }) {
        println!("@T: {}", transform.position.x)
    }

    renderer.render();
}
