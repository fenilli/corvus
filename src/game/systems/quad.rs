use crate::{
    game::components::{Mesh, Quad},
    renderer::{Renderer, Vertex},
    world::{CommandBuffer, World},
};

pub fn quad_system(world: &mut World, renderer: &mut Renderer) {
    let mut command_buffer = CommandBuffer::new();

    {
        let Some(quads) = world.components::<Quad>() else {
            return;
        };

        let Some(meshes) = world.components::<Mesh>() else {
            return;
        };

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

        for (entity, quad) in world
            .entities()
            .zip(quads.iter())
            .zip(meshes.iter())
            .filter_map(|((entity, quad), mesh)| match (mesh, quad) {
                (Some(_), _) => None,
                (None, Some(quad)) => Some((entity, quad)),
                (_, _) => None,
            })
        {
            let handle = renderer.create_mesh(
                format!("quad_{}_{}", quad.height, quad.width),
                vertices,
                indices,
            );

            command_buffer.schedule(move |world| world.insert(entity, Mesh { handle }));
        }
    }

    command_buffer.execute(world);
}
