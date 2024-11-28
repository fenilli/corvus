use crate::{
    game::components::Quad,
    renderer::{Renderer, Vertex},
    world::World,
};

pub fn quad_system(world: &World, renderer: &mut Renderer) {
    let quads = world.get_components::<Quad>().unwrap();
    let iter = quads.iter();

    let vertices = &[
        Vertex {
            position: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            uv: [0.0, 1.0],
        },
    ];
    let indices = &[0, 1, 2, 2, 3, 0];

    for quad in iter {
        let handle = renderer.create_mesh(
            format!("quad_{}_{}", quad.height, quad.width),
            vertices,
            indices,
        );

        println!("{}", handle);
    }
}
