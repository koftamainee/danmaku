pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
}

pub struct BufferDescriptor<'a> {
    pub usage: BufferUsage,
    pub size: u64,
    pub label: Option<&'a str>,
}

pub struct Buffer {
    pub(crate) wgpu_buffer: wgpu::Buffer,
    pub(crate) size: u64,
    pub(crate) usage: BufferUsage,
}
