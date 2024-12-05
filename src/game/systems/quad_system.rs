use crate::{
    game::components::{MeshComponent, QuadComponent},
    renderer::Vertex,
    world::World,
};

pub fn quad_system(world: &mut World) {
    let mut command_buffer = World::command_buffer();

    {
        let Some(quads) = world.components::<QuadComponent>() else {
            return;
        };
        let Some(meshes) = world.components::<MeshComponent>() else {
            return;
        };

        for (entity, quad) in world
            .entities()
            .zip(quads.iter())
            .zip(meshes.iter())
            .filter_map(|((entity, quad), mesh)| match (quad, mesh) {
                (_, Some(_)) => None,
                (Some(quad), None) => Some((entity, quad)),
                (_, _) => None,
            })
        {
            let half_w = quad.width / 2.0;
            let half_h = quad.height / 2.0;

            let vertices = &[
                Vertex {
                    position: [-half_w, -half_h, 0.0],
                    color: [255.0, 255.0, 255.0],
                },
                Vertex {
                    position: [half_w, -half_h, 0.0],
                    color: [255.0, 255.0, 255.0],
                },
                Vertex {
                    position: [half_w, half_h, 0.0],
                    color: [255.0, 255.0, 255.0],
                },
                Vertex {
                    position: [-half_w, half_h, 0.0],
                    color: [255.0, 255.0, 255.0],
                },
            ];
            let indices = &[0, 1, 2, 2, 3, 0];

            let mesh_component = MeshComponent {
                vertices: vertices.to_vec(),
                indices: indices.to_vec(),
            };

            command_buffer.schedule(move |world| {
                world.insert(entity, mesh_component);
            });
        }
    }

    command_buffer.execute(world);
}
