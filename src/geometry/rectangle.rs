use crate::{render::Vertex, resources::Mesh};

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Mesh {
        let hw = width / 2.;
        let hh = height / 2.;

        Mesh::from_data(
            vec![
                Vertex::new([-hw, -hh], [0., 0.]),
                Vertex::new([hw, -hh], [0., 0.]),
                Vertex::new([hw, hh], [0., 0.]),
                Vertex::new([-hw, hh], [0., 0.]),
            ],
            vec![0, 1, 2, 2, 3, 0],
        )
    }
}
