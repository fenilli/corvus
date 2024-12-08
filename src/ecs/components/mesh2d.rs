use crate::resources::{AssetHandle, Mesh};

pub struct Mesh2d {
    pub handle: AssetHandle<Mesh>,
}

impl Mesh2d {
    pub fn new(handle: AssetHandle<Mesh>) -> Self {
        Self { handle }
    }
}
