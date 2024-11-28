mod cache;
mod material;
mod mesh;
mod texture;
mod vertex;

use pollster::FutureExt;
use std::sync::Arc;
use wgpu::Instance;
use winit::window::Window;

use cache::Cache;
use material::Material;
use mesh::Mesh;
use texture::Texture;
pub use vertex::Vertex;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    mesh_caches: Cache<Mesh>,
    texture_caches: Cache<Texture>,
    material_caches: Cache<Material>,

    window: Arc<Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();
        let instance = Instance::new(Default::default());

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&Default::default())
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block_on()
            .unwrap();

        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface_config.format = wgpu::TextureFormat::Bgra8UnormSrgb;
        surface.configure(&device, &surface_config);

        Self {
            device,
            queue,

            mesh_caches: Cache::new(),
            texture_caches: Cache::new(),
            material_caches: Cache::new(),

            window,
        }
    }

    pub fn load_texture(&mut self, name: String, path: String) -> String {
        if self.texture_caches.exists(&path) {
            path
        } else {
            let texture = Texture::new(path, &self.device);

            self.texture_caches.insert(name, texture)
        }
    }

    pub fn create_texture(
        &mut self,
        name: String,
        texture: &Texture,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> String {
        if self.material_caches.exists(&name) {
            name
        } else {
            let material = Material::new(texture, &self.device, bind_group_layout);

            self.material_caches.insert(name, material)
        }
    }

    pub fn create_mesh(&mut self, name: String, vertices: &[Vertex], indices: &[u32]) -> String {
        if self.mesh_caches.exists(&name) {
            name
        } else {
            let mesh = Mesh::new(vertices, indices, &self.device);

            self.mesh_caches.insert(name, mesh)
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
