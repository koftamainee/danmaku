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
pub mod atlas;

pub use crate::error::RendererError;
pub use crate::graphics_device::{GraphicsDevice, Frame};
pub use crate::color::Color;
pub use crate::texture::{Texture, TextureFormat, TextureDescriptor, Rect, TextureLoadError, TextureLoadContext};
pub use crate::buffer::{Buffer, BufferDescriptor, BufferUsage};
pub use crate::sampler::{Sampler, SamplerDescriptor, FilterMode, AddressMode};
pub use crate::pipeline::{Pipeline, PipelineDescriptor, BlendMode, StepMode, VertexLayout, VertexAttribute, VertexFormat};
pub use crate::bind_group::{BindGroup, BindGroupEntry};
pub use crate::render_pass::RenderPass;
pub use crate::camera::{Camera, OrthographicCamera};
pub use crate::sprite_batch::{SpriteBatch, BeginParams, DrawParams, SpriteEffects, BatchStats, SpriteBatchError};
pub use crate::atlas::{Atlas, AtlasLoadContext, AtlasError, SpriteRegion, CollisionShape};
pub use wgpu::IndexFormat;

pub const BATCH_SHADER: &str = include_str!("shaders/batch.wgsl");
