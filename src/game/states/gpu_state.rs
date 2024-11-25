use std::sync::Arc;

use winit::window::Window;

pub struct GpuState {
    window: Arc<Window>,
}

impl GpuState {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);

        Self { window }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
