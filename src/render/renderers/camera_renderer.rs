use crate::render::GraphicsDevice;

pub struct CameraRenderer {
    world_to_projection_uniform_buffer: wgpu::Buffer,
    pub world_to_projection_bind_group_layout: wgpu::BindGroupLayout,
    world_to_projection_bind_group: wgpu::BindGroup,
}

impl CameraRenderer {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let world_to_projection_bind_group_layout = graphics_device
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("View Projection"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let world_to_projection_uniform_buffer =
            graphics_device
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("View Projection Uniform Buffer"),
                    size: std::mem::size_of::<glam::Mat4>() as wgpu::BufferAddress,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

        let world_to_projection_bind_group =
            graphics_device
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("View Projection Bind Group"),
                    layout: &world_to_projection_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            world_to_projection_uniform_buffer.as_entire_buffer_binding(),
                        ),
                    }],
                });

        Self {
            world_to_projection_uniform_buffer,
            world_to_projection_bind_group_layout,
            world_to_projection_bind_group,
        }
    }

    pub fn prepare(&mut self, graphics_device: &GraphicsDevice, world_to_projection: glam::Mat4) {
        graphics_device.queue.write_buffer(
            &self.world_to_projection_uniform_buffer,
            0,
            bytemuck::bytes_of(&world_to_projection),
        );
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, &self.world_to_projection_bind_group, &[]);
    }
}
