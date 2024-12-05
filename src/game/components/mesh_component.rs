use crate::renderer::Vertex;

#[derive(Debug)]
pub struct MeshComponent {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
