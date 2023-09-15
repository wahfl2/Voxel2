use wgpu::{Device, TextureFormat, PipelineLayout, RenderPipeline, PipelineLayoutDescriptor, RenderPipelineDescriptor};

use super::shaders::Shaders;

pub struct Pipeline {
    pub layout: PipelineLayout,
    pub inner: RenderPipeline,
}

impl Pipeline {
    pub fn new(device: &Device, shaders: &Shaders, format: TextureFormat) -> Self {
        let layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let inner = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),

            vertex: wgpu::VertexState {
                module: &shaders.shader.inner,
                entry_point: "vs_main",
                buffers: &[],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shaders.shader.inner,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { layout, inner }
    }
}