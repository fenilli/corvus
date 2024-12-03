use pollster::FutureExt;
use std::sync::Arc;
use wgpu::Instance;
use winit::window::Window;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

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

            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
