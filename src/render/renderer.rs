use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use super::GpuContext;

pub struct Renderer {
    gpu: GpuContext,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        Self {
            gpu: GpuContext::new(window),
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let gpu = &mut self.gpu;

        gpu.surface_config.width = size.width;
        gpu.surface_config.height = size.height;

        gpu.surface.configure(&gpu.device, &gpu.surface_config);
    }
}
