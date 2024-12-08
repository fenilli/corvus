use crate::resources::Mesh;

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Mesh {
        let hw = width / 2.;
        let hh = height / 2.;

        Mesh::from_data(
            vec![[-hw, -hh, 0.], [hw, -hh, 0.], [hw, hh, 0.], [-hw, hh, 0.]],
            vec![0, 1, 2, 2, 3, 0],
        )
    }
}
