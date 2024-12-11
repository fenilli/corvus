use crate::{
    ecs::components::{Rectangle, Sprite, Transform},
    render::{Instance, Renderer},
    World,
};

pub fn render_system(world: &mut World, renderer: &mut Renderer) {
    let mut vertex_data: Vec<Instance> = Vec::new();

    for (transform, rectangle) in world.entities().filter_map(|entity| {
        let (Some(_), Some(transform), Some(rectangle)) = (
            world.get_component::<Sprite>(entity),
            world.get_component::<Transform>(entity),
            world.get_component::<Rectangle>(entity),
        ) else {
            return None;
        };

        Some((transform, rectangle))
    }) {
        vertex_data.push(Instance::new(
            transform.position.into(),
            [rectangle.width as f32, rectangle.height as f32],
            [1.0, 1.0, 1.0],
        ));
    }

    renderer.render(vertex_data.as_slice());
}
