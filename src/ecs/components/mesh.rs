use crate::resources::AssetHandle;

pub struct Mesh {
    pub handle: AssetHandle,
}

impl Mesh {
    pub fn new(handle: AssetHandle) -> Self {
        Self { handle }
    }
}
