mod cache;
mod material;
mod mesh;
mod texture;
mod vertex;

use cache::Cache;
use material::Material;
use mesh::Mesh;
use texture::Texture;
use vertex::Vertex;

pub struct Renderer {
    mesh_caches: Cache<Mesh>,
    texture_caches: Cache<Texture>,
    material_caches: Cache<Material>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            mesh_caches: Cache::new(),
            texture_caches: Cache::new(),
            material_caches: Cache::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        name: &'static str,
        path: &'static str,
        device: &wgpu::Device,
    ) -> &'static str {
        let texture = Texture::new(path, device);

        self.texture_caches.insert(name, texture)
    }

    pub fn create_texture(
        &mut self,
        name: &'static str,
        texture: &Texture,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> &'static str {
        let material = Material::new(texture, device, bind_group_layout);

        self.material_caches.insert(name, material)
    }

    pub fn create_mesh(
        &mut self,
        name: &'static str,
        vertices: &[Vertex],
        indices: &[u32],
        device: &wgpu::Device,
    ) -> &'static str {
        let mesh = Mesh::new(vertices, indices, device);

        self.mesh_caches.insert(name, mesh)
    }
}
