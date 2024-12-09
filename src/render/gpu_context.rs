use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

pub struct GpuContext {
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,

    pub window: Arc<Window>,
}

impl GpuContext {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .block_on()
            .unwrap();

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

        Self {
            surface,
            surface_config,
            device,
            queue,

            window,
        }
    }
}
