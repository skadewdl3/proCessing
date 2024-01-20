use wgpu::{Buffer, ShaderModule, RenderPipeline, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, RenderPipelineDescriptor, VertexState, ShaderModuleDescriptor, ShaderSource, VertexBufferLayout, FragmentState};

use crate::renderer::vertex::Vertex;

use super::state::get_renderer_state;

pub struct Shader {
    pub module: ShaderModule,
    pub pipeline: RenderPipeline,
    pub draw_count: u32,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,

    pub has_vertex_buffer: bool,
    pub has_index_buffer: bool,
    pub has_uniforms: bool
}

#[derive(Default)]
pub struct ShaderBuilder {
    label: String,
    content: Option<String>,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    vertex_or_index_count: u32,

    has_vertex_buffer: bool,
    has_index_buffer: bool,
}

impl ShaderBuilder {
    pub fn new () -> Self {
        Self {
            label: String::from("Shader Builder"),
            ..Default::default()
        }
    }

    pub fn with_label (&mut self, label: impl Into<String>) -> &mut Self {
        self.label = label.into();
        self
    }

    pub fn with_content (&mut self, content: impl Into<String>) -> &mut Self {
        self.content = Some(content.into());
        self
    }

    pub fn from_source (&mut self, source: impl Into<String>) -> &mut Self {
        self.content = 
            if let Ok(content_string) = 
                std::fs::read_to_string(source.into()) { Some(content_string) }
            else { None };

        self
    }

    pub fn with_vertex_buffer (&mut self, vertices: Vec<Vertex>) -> &mut Self {
        let state = get_renderer_state();
        let device = state.device.as_ref().unwrap();

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(""),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX
        });

        self.vertex_buffer = Some(vertex_buffer);
        self.has_vertex_buffer = true;

        if !self.has_index_buffer { self.vertex_or_index_count = vertices.len() as u32; }

        self
    }

    pub fn with_index_buffer (&mut self, indices: Vec<u32>) -> &mut Self {
        let state = get_renderer_state();
        let device = state.device.as_ref().unwrap();

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(""),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX
        });
        
        self.index_buffer = Some(index_buffer);
        self.has_index_buffer = true;

        self.vertex_or_index_count = indices.len() as u32;

        self
    }

    pub fn build (&mut self) -> Shader {
        // 
        let state = get_renderer_state();
        let device = state.device.as_ref().unwrap();

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(self.label.as_str()),
            source: ShaderSource::Wgsl(self.content.as_ref().unwrap().into())
        });

        // make pipeline layout

        let mut buffers: Vec<VertexBufferLayout> = vec![];
        if self.vertex_buffer.is_some() {
            let x = VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3]
            };
            buffers.push(x);
        }

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(self.label.as_str()),

            layout: None,

            vertex: VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &buffers
            },

            fragment: Some(FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
					format: wgpu::TextureFormat::Bgra8UnormSrgb,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::REPLACE,
						alpha: wgpu::BlendComponent::REPLACE,
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });


        Shader {
            module,
            pipeline,
            draw_count: self.vertex_or_index_count,

            has_vertex_buffer: self.has_vertex_buffer,
            has_index_buffer: self.has_index_buffer,
            has_uniforms: false,

            vertex_buffer: self.vertex_buffer.take(),
            index_buffer: self.index_buffer.take()
        }
    }
}