use crate::{app::components::Camera, ecs::World, render::Renderer};

pub struct CameraSystem;

impl CameraSystem {
    pub fn prepare_projection(world: &World, renderer: &mut Renderer) {
        let Some(camera) = world.single::<Camera>() else {
            panic!("we need a camera with projection to show to the screen.");
        };

        let view_projection = camera.get_view_projection(glam::Vec2::new(0.0, 0.0));
        renderer.set_view_projection(view_projection);
    }
}
