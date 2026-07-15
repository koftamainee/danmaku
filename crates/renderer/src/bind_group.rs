use crate::texture::GpuTexture;
use crate::buffer::Buffer;
use crate::sampler::Sampler;

pub enum BindGroupEntry<'a> {
    Texture {
        texture: &'a GpuTexture,
        sampler: &'a Sampler,
    },
    UniformBuffer {
        buffer: &'a Buffer,
    },
}

pub struct BindGroup {
    pub(crate) wgpu_bind_group: wgpu::BindGroup,
    pub(crate) layout: wgpu::BindGroupLayout,
}
