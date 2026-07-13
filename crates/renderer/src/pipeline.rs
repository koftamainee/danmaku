#[derive(Clone)]
pub enum BlendMode {
    Opaque,
    AlphaBlend,
    Additive,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepMode {
    Vertex,
    Instance,
}

pub struct VertexLayout {
    pub stride: u32,
    pub step_mode: StepMode,
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Clone)]
pub struct VertexAttribute {
    pub location: u32,
    pub format: VertexFormat,
    pub offset: u32,
}

#[derive(Clone)]
pub enum VertexFormat {
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
}

pub struct PipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader_source: String,
    pub blend_mode: BlendMode,
    pub vertex_layouts: Vec<VertexLayout>,
    pub bind_group_layouts: &'a [&'a crate::bind_group::BindGroup],
}

pub struct Pipeline {
    pub(crate) wgpu_pipeline: wgpu::RenderPipeline,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}
