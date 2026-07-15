mod graphics_device;
mod error;
mod color;
mod sprite_batch;
mod texture;
mod buffer;
mod sampler;
mod pipeline;
mod bind_group;
mod render_pass;
mod camera;

pub use crate::error::RendererError;
pub use crate::graphics_device::{GraphicsDevice, Frame};
pub use crate::color::Color;
pub use crate::texture::{GpuTexture, GpuTextureLoadContext, GpuTextureError, TextureFormat, TextureDescriptor, Rect, Hitbox};
pub use crate::buffer::{Buffer, BufferDescriptor, BufferUsage};
pub use crate::sampler::{Sampler, SamplerDescriptor, FilterMode, AddressMode};
pub use crate::pipeline::{Pipeline, PipelineDescriptor, BlendMode, StepMode, VertexLayout, VertexAttribute, VertexFormat};
pub use crate::bind_group::{BindGroup, BindGroupEntry};
pub use crate::render_pass::RenderPass;
pub use crate::camera::{Camera, OrthographicCamera};
pub use crate::sprite_batch::{SpriteBatch, BeginParams, DrawParams, SpriteEffects, BatchStats, SpriteBatchError};
pub use wgpu::IndexFormat;

pub const BATCH_SHADER: &str = include_str!("shaders/batch.wgsl");
