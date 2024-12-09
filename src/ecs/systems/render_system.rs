use crate::{render::Renderer, World};

pub fn render_system(world: &mut World, renderer: &mut Renderer) {
    renderer.render();
}
