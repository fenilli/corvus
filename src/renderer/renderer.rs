use pollster::FutureExt;
use std::sync::Arc;
use wgpu::Instance;
use winit::window::Window;

use super::{cache::Cache, material::Material, mesh::Mesh, texture::Texture, vertex::Vertex};

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    mesh_caches: Cache<Mesh>,
    texture_caches: Cache<Texture>,
    material_caches: Cache<Material>,

    material_bind_group_layout: wgpu::BindGroupLayout,
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

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    },
                ],
            });

        Self {
            device,
            queue,

            mesh_caches: Cache::new(),
            texture_caches: Cache::new(),
            material_caches: Cache::new(),

            material_bind_group_layout,

            window,
        }
    }

    pub fn create_texture(&mut self, name: String, path: String) -> String {
        if self.texture_caches.exists(&path) {
            path
        } else {
            let texture = Texture::new(path, &self.device);

            self.texture_caches.insert(name, texture)
        }
    }

    pub fn create_material(&mut self, name: String, texture_handle: String) -> String {
        if self.material_caches.exists(&name) {
            name
        } else {
            let texture = self.texture_caches.get(texture_handle).unwrap();
            let material = Material::new(&texture, &self.device, &self.material_bind_group_layout);

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
