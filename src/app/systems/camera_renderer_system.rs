use crate::{
    app::components::Camera,
    ecs::World,
    render::{renderers::CameraRenderer, GraphicsDevice},
};

pub struct CameraRendererSystem {
    camera_renderer: CameraRenderer,
}

impl CameraRendererSystem {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        Self {
            camera_renderer: CameraRenderer::new(graphics_device),
        }
    }

    pub fn binding(&self) -> &wgpu::BindGroupLayout {
        &self.camera_renderer.world_to_projection_bind_group_layout
    }

    pub fn prepare(&mut self, world: &World, graphics_device: &GraphicsDevice) {
        let world_to_projection = world
            .entities()
            .find_map(|entity| {
                if let Some(camera) = world.get_component::<Camera>(entity) {
                    Some(camera.world_to_projection())
                } else {
                    None
                }
            })
            .expect("The game needs a camera.");

        self.camera_renderer
            .prepare(graphics_device, world_to_projection);
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.camera_renderer.render(render_pass);
    }
}
