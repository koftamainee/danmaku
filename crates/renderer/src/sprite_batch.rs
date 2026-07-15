use std::collections::HashMap;

use glam::{Mat4, Vec2};

use crate::bind_group::{BindGroup, BindGroupEntry};
use crate::buffer::{BufferDescriptor, Buffer, BufferUsage};
use crate::color::Color;
use crate::error::RendererError;
use crate::graphics_device::Frame;
use crate::pipeline::{self, BlendMode, Pipeline, PipelineDescriptor, StepMode, VertexAttribute, VertexFormat, VertexLayout};
use crate::sampler::{AddressMode, FilterMode, Sampler, SamplerDescriptor};
use crate::texture::{Rect, GpuTexture};
use crate::GraphicsDevice;
use crate::BATCH_SHADER;

const QUAD_VERTICES: [f32; 16] = [
    -0.5, -0.5, 0.0, 1.0,
     0.5, -0.5, 1.0, 1.0,
     0.5,  0.5, 1.0, 0.0,
    -0.5,  0.5, 0.0, 0.0,
];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const INSTANCE_SIZE: usize = 52;

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct InstanceData {
    position: [f32; 2],
    size: [f32; 2],
    uv: [f32; 4],
    color: [f32; 4],
    rotation: f32,
}

pub struct SpriteBatch {
    quad_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    instance_capacity: usize,
    globals_buffer: Buffer,
    globals_bind_group: BindGroup,
    pipelines: [Option<Pipeline>; 3],

    pending_ids: Vec<u64>,
    pending_data: Vec<InstanceData>,

    texture_cache: HashMap<u64, BindGroup>,

    is_batching: bool,
    current_blend: pipeline::BlendMode,
    current_sampler: Sampler,
    current_viewport: Option<Rect>,
}

pub struct BeginParams {
    pub view_projection: Mat4,
    pub blend_mode: BlendMode,
    pub viewport: Option<Rect>,
    pub sampler: FilterMode,
}

pub struct DrawParams<'a> {
    pub texture: &'a GpuTexture,
    pub source: Option<Rect>,
    pub position: Vec2,
    pub origin: Option<Vec2>,
    pub rotation: f32,
    pub scale: Vec2,
    pub color: Color,
    pub effects: SpriteEffects,
}

impl<'a> DrawParams<'a> {
    pub fn new(texture: &'a GpuTexture) -> Self {
        Self {
            texture,
            source: None,
            position: Vec2::ZERO,
            origin: None,
            rotation: 0.0,
            scale: Vec2::ONE,
            color: Color::WHITE,
            effects: SpriteEffects::NONE,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteEffects(u32);

impl SpriteEffects {
    pub const NONE: Self = Self(0);
    pub const FLIP_HORIZONTAL: Self = Self(1);
    pub const FLIP_VERTICAL: Self = Self(2);

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for SpriteEffects {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub struct BatchStats {
    pub draw_calls: u32,
    pub sprites: u32,
    pub render_passes: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum SpriteBatchError {
    #[error("sprites must be drawn between begin() and end()")]
    NotBatching,
    #[error("begin() was called without calling end() first")]
    AlreadyBatching,
}

impl SpriteBatch {
    pub fn new(gd: &GraphicsDevice) -> Result<Self, RendererError> {
        Self::with_capacity(gd, 1000)
    }

    pub fn with_capacity(gd: &GraphicsDevice, capacity: usize) -> Result<Self, RendererError> {
        let quad_buffer = gd.create_buffer(BufferDescriptor {
            label: Some("SpriteBatch quad"),
            size: std::mem::size_of_val(&QUAD_VERTICES) as u64,
            usage: BufferUsage::Vertex,
        });
        gd.write_buffer(&quad_buffer, 0, bytemuck::cast_slice(&QUAD_VERTICES));

        let index_buffer = gd.create_buffer(BufferDescriptor {
            label: Some("SpriteBatch index"),
            size: std::mem::size_of_val(&QUAD_INDICES) as u64,
            usage: BufferUsage::Index,
        });
        gd.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&QUAD_INDICES));

        let instance_buffer = gd.create_buffer(BufferDescriptor {
            label: Some("SpriteBatch instances"),
            size: (capacity * INSTANCE_SIZE) as u64,
            usage: BufferUsage::Vertex,
        });

        let globals_buffer = gd.create_buffer(BufferDescriptor {
            label: Some("SpriteBatch globals"),
            size: 64,
            usage: BufferUsage::Uniform,
        });

        let globals_bind_group = gd.create_bind_group(Some("SpriteBatch globals"), &[
            BindGroupEntry::UniformBuffer { buffer: &globals_buffer },
        ]);

        let placeholder_texture = gd.create_texture(
            crate::texture::TextureDescriptor {
                label: Some("SpriteBatch placeholder"),
                width: 1,
                height: 1,
                format: crate::texture::TextureFormat::Rgba8UnormSrgb,
            },
            None,
        );
        let placeholder_sampler = gd.create_sampler(SamplerDescriptor {
            label: Some("SpriteBatch placeholder sampler"),
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
        });
        let template_texture_bind_group = gd.create_bind_group(Some("SpriteBatch template texture"), &[
            BindGroupEntry::Texture {
                texture: &placeholder_texture,
                sampler: &placeholder_sampler,
            },
        ]);

        let globals_bind_group_for_pipeline = gd.create_bind_group(Some("SpriteBatch pipeline globals"), &[
            BindGroupEntry::UniformBuffer { buffer: &globals_buffer },
        ]);

        let make_pipeline = |blend: BlendMode| -> Result<Option<Pipeline>, RendererError> {
            Ok(Some(gd.create_pipeline(PipelineDescriptor {
                label: None,
                shader_source: BATCH_SHADER.to_string(),
                blend_mode: blend,
                vertex_layouts: vec![
                    VertexLayout {
                        stride: 16,
                        step_mode: StepMode::Vertex,
                        attributes: vec![
                            VertexAttribute { location: 0, format: VertexFormat::Float32x2, offset: 0 },
                            VertexAttribute { location: 1, format: VertexFormat::Float32x2, offset: 8 },
                        ],
                    },
                    VertexLayout {
                        stride: 52,
                        step_mode: StepMode::Instance,
                        attributes: vec![
                            VertexAttribute { location: 2, format: VertexFormat::Float32x2, offset: 0 },
                            VertexAttribute { location: 3, format: VertexFormat::Float32x2, offset: 8 },
                            VertexAttribute { location: 4, format: VertexFormat::Float32x4, offset: 16 },
                            VertexAttribute { location: 5, format: VertexFormat::Float32x4, offset: 32 },
                            VertexAttribute { location: 6, format: VertexFormat::Float32, offset: 48 },
                        ],
                    },
                ],
                bind_group_layouts: &[&globals_bind_group_for_pipeline, &template_texture_bind_group],
            })))
        };

        let pipelines = [
            make_pipeline(BlendMode::Opaque)?,
            make_pipeline(BlendMode::AlphaBlend)?,
            make_pipeline(BlendMode::Additive)?,
        ];

        let current_sampler = gd.create_sampler(SamplerDescriptor {
            label: None,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
        });

        Ok(Self {
            quad_buffer,
            index_buffer,
            instance_buffer,
            instance_capacity: capacity,
            globals_buffer,
            globals_bind_group,
            pipelines,

            pending_ids: Vec::new(),
            pending_data: Vec::new(),

            texture_cache: HashMap::new(),

            is_batching: false,
            current_blend: BlendMode::AlphaBlend,
            current_sampler,
            current_viewport: None,
        })
    }

    pub fn begin(&mut self, gd: &GraphicsDevice, params: BeginParams) {
        assert!(!self.is_batching, "begin() called while already batching");
        self.is_batching = true;
        self.current_blend = params.blend_mode;
        self.current_viewport = params.viewport;

        self.pending_ids.clear();
        self.pending_data.clear();

        let filter = params.sampler;
        self.current_sampler = gd.create_sampler(SamplerDescriptor {
            label: None,
            mag_filter: filter,
            min_filter: filter,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
        });

        gd.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::cast_slice(&params.view_projection.to_cols_array()),
        );
    }

    pub fn draw(&mut self, gd: &GraphicsDevice, params: DrawParams<'_>) {
        assert!(self.is_batching, "draw() called outside of begin()/end()");

        let tex_id = params.texture.id();
        self.ensure_bind_group(gd, tex_id, params.texture);
        let tex_w = params.texture.width() as f32;
        let tex_h = params.texture.height() as f32;

        let (src_x, src_y, src_w, src_h) = match params.source {
            Some(rect) => (rect.x, rect.y, rect.width, rect.height),
            None => (0.0, 0.0, tex_w, tex_h),
        };

        let mut u0 = src_x / tex_w;
        let mut v0 = src_y / tex_h;
        let mut u1 = (src_x + src_w) / tex_w;
        let mut v1 = (src_y + src_h) / tex_h;

        if params.effects.contains(SpriteEffects::FLIP_HORIZONTAL) {
            std::mem::swap(&mut u0, &mut u1);
        }
        if params.effects.contains(SpriteEffects::FLIP_VERTICAL) {
            std::mem::swap(&mut v0, &mut v1);
        }

        let size = [src_w * params.scale.x, src_h * params.scale.y];

        let origin = params.origin.unwrap_or(Vec2::new(0.5, 0.5));
        let offset = (Vec2::new(0.5, 0.5) - origin) * Vec2::new(size[0], size[1]);

        let c = params.rotation.cos();
        let s = params.rotation.sin();
        let rotated_offset = Vec2::new(
            offset.x * c - offset.y * s,
            offset.x * s + offset.y * c,
        );

        let position = params.position + rotated_offset;

        self.pending_ids.push(tex_id);
        self.pending_data.push(InstanceData {
            position: position.to_array(),
            size,
            uv: [u0, v0, u1, v1],
            color: params.color.to_array(),
            rotation: params.rotation,
        });
    }

    fn group_consecutive(ids: &[u64]) -> Vec<(u64, u32)> {
        if ids.is_empty() {
            return Vec::new();
        }

        let mut groups = Vec::new();
        let mut current_id = ids[0];
        let mut count = 1u32;

        for &id in &ids[1..] {
            if id == current_id {
                count += 1;
            } else {
                groups.push((current_id, count));
                current_id = id;
                count = 1;
            }
        }
        groups.push((current_id, count));

        groups
    }

    pub fn end(&mut self, gd: &GraphicsDevice, frame: &mut Frame) -> Result<BatchStats, SpriteBatchError> {
        if !self.is_batching {
            return Err(SpriteBatchError::NotBatching);
        }
        self.is_batching = false;

        if self.pending_data.is_empty() {
            return Ok(BatchStats {
                draw_calls: 0,
                sprites: 0,
                render_passes: 0,
            });
        }

        let sprite_count = self.pending_data.len() as u32;

        if self.pending_data.len() > self.instance_capacity {
            self.instance_capacity = self.pending_data.len() * 2;
            self.instance_buffer = gd.create_buffer(BufferDescriptor {
                label: Some("SpriteBatch instances"),
                size: (self.instance_capacity * INSTANCE_SIZE) as u64,
                usage: BufferUsage::Vertex,
            });
        }

        gd.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.pending_data),
        );

        let mut pass = frame.begin_pass();

        if let Some(viewport) = &self.current_viewport {
            pass.set_viewport(viewport);
        }

        let pipeline_index = match self.current_blend {
            BlendMode::Opaque => 0,
            BlendMode::AlphaBlend => 1,
            BlendMode::Additive => 2,
        };
        pass.set_pipeline(self.pipelines[pipeline_index].as_ref().unwrap());

        pass.set_bind_group(0, &self.globals_bind_group);
        pass.set_vertex_buffer(0, &self.quad_buffer);
        pass.set_vertex_buffer(1, &self.instance_buffer);
        pass.set_index_buffer(&self.index_buffer, wgpu::IndexFormat::Uint16);

        let groups = Self::group_consecutive(&self.pending_ids);

        for (tex_id, count) in &groups {
            let bind_group = &self.texture_cache[tex_id];
            pass.set_bind_group(1, bind_group);
            pass.draw_indexed(6, *count);
        }

        pass.finish();

        let stats = BatchStats {
            draw_calls: groups.len() as u32,
            sprites: sprite_count,
            render_passes: 1,
        };

        self.pending_ids.clear();
        self.pending_data.clear();

        Ok(stats)
    }

    fn ensure_bind_group(&mut self, gd: &GraphicsDevice, tex_id: u64, texture: &GpuTexture) {
        if !self.texture_cache.contains_key(&tex_id) {
            let bind_group = gd.create_bind_group(Some("SpriteBatch texture"), &[
                BindGroupEntry::Texture { texture, sampler: &self.current_sampler },
            ]);
            self.texture_cache.insert(tex_id, bind_group);
        }
    }

    pub fn remove_cached_bind_group(&mut self, texture_id: u64) {
        self.texture_cache.remove(&texture_id);
    }
}
