pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_data(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}
