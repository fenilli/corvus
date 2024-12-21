use glam::Mat4;
use wgpu::util::DeviceExt;

use super::{vertex::Vertex, GraphicsDevice, Instance};

pub struct QuadRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    projection_uniform_buffer: wgpu::Buffer,
    projection_bind_group: wgpu::BindGroup,
    index_len: u32,
    instances_len: u32,
}

impl QuadRenderer {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let shader = graphics_device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/quad.wgsl").into()),
            });

        let view_projection_bind_group_layout =
            graphics_device
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

        let projection_uniform_buffer =
            graphics_device
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("View Projection Uniform Buffer"),
                    size: std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

        let projection_bind_group =
            graphics_device
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("View Projection Bind Group"),
                    layout: &view_projection_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            projection_uniform_buffer.as_entire_buffer_binding(),
                        ),
                    }],
                });

        let render_pipeline_layout =
            graphics_device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&view_projection_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            graphics_device
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Vertex::desc(), Instance::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics_device.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        let vertices: &[Vertex; 4] = &[
            Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0]),
            Vertex::new([0.5, 0.5], [0.0, 0.0, 1.0]),
            Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([-0.5, -0.5], [1.0, 0.0, 0.0]),
        ];

        let vertex_buffer =
            graphics_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let indices: &[u16] = &[0, 1, 2, 2, 3, 0];

        let index_buffer =
            graphics_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let instance_buffer = graphics_device
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (std::mem::size_of::<Instance>() * 100) as wgpu::BufferAddress, // Max 100 instances
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            projection_uniform_buffer,
            projection_bind_group,
            index_len: indices.len() as u32,
            instances_len: 0,
        }
    }

    pub fn prepare(
        &mut self,
        graphics_device: &GraphicsDevice,
        projection_matrix: Mat4,
        instances: &[Instance],
    ) {
        self.instances_len = instances.len() as u32;
        graphics_device.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instances),
        );
        graphics_device.queue.write_buffer(
            &self.projection_uniform_buffer,
            0,
            bytemuck::bytes_of(&projection_matrix),
        );
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.projection_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_len, 0, 0..self.instances_len);
    }
}
