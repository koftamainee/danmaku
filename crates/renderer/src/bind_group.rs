use crate::texture::Texture;
use crate::buffer::Buffer;
use crate::sampler::Sampler;

pub enum BindGroupEntry<'a> {
    Texture {
        texture: &'a Texture,
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
