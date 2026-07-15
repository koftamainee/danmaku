use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::RendererError;
use crate::color::Color;
use crate::texture::{GpuTexture, TextureDescriptor};
use crate::buffer::{BufferDescriptor, Buffer};
use crate::sampler::{SamplerDescriptor, Sampler};
use crate::pipeline::{PipelineDescriptor, Pipeline};
use crate::bind_group::{BindGroupEntry, BindGroup};

use crate::render_pass::RenderPass;

pub struct GraphicsDevice {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    next_texture_id: AtomicU64,
}

pub struct Frame {
    surface_texture: wgpu::SurfaceTexture,
    texture_view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
    clear_color: Option<wgpu::Color>,
    pass_count: u32,
}

impl GraphicsDevice {
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
        vsync: bool,
    ) -> Result<Self, RendererError> {
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        };
        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(window)?;

        let adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            apply_limit_buckets: false,
        };

        let adapter = instance.request_adapter(&adapter_options).await?;

        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("Graphics device"),
            required_features: Default::default(),
            required_limits: Default::default(),
            experimental_features: Default::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await?;

        let surface_config = Self::get_initial_surface_config(
            &surface.get_capabilities(&adapter),
            width,
            height,
            vsync,
        );

        surface.configure(&device, &surface_config);

        Ok(Self {
            instance,
            surface,

            surface_config,
            adapter,
            device,
            queue,

            next_texture_id: AtomicU64::new(1),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn surface_size(&self) -> (u32, u32) {
        (self.surface_config.width, self.surface_config.height)
    }

    pub fn begin_frame(&mut self) -> Result<Frame, RendererError> {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(RendererError::FrameSkipped);
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                return Err(RendererError::SurfaceOutdated);
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RendererError::ValidationError);
            }
        };

        let texture_view = surface_texture.texture
            .create_view(&Default::default());
        let encoder = self.device.
            create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

        Ok(Frame {
            surface_texture,
            texture_view,
            encoder,
            clear_color: None,
            pass_count: 0,
        })
    }

    pub fn end_frame(&mut self, frame: Frame) {
        let mut frame = frame;
        if frame.pass_count == 0 {
            if let Some(color) = frame.clear_color.take() {
                let pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.texture_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });
                drop(pass);
            }
        }
        self.queue.submit(std::iter::once(frame.encoder.finish()));
        self.queue.present(frame.surface_texture);
    }

    pub fn create_buffer(&self, desc: BufferDescriptor) -> Buffer {
        let wgpu_usage = match desc.usage {
            crate::buffer::BufferUsage::Vertex => wgpu::BufferUsages::VERTEX,
            crate::buffer::BufferUsage::Index => wgpu::BufferUsages::INDEX,
            crate::buffer::BufferUsage::Uniform => wgpu::BufferUsages::UNIFORM,
        };

        let wgpu_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: desc.label,
            size: desc.size,
            usage: wgpu_usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Buffer {
            wgpu_buffer,
            size: desc.size,
            usage: desc.usage,
        }
    }

    pub fn write_buffer(&self, buffer: &Buffer, offset: u64, data: &[u8]) {
        self.queue.write_buffer(&buffer.wgpu_buffer, offset, data);
    }

    pub fn create_texture(&self, desc: TextureDescriptor, data: Option<&[u8]>) -> GpuTexture {
        let wgpu_format = match desc.format {
            crate::texture::TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            crate::texture::TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        };

        let size = wgpu::Extent3d {
            width: desc.width,
            height: desc.height,
            depth_or_array_layers: 1,
        };

        let wgpu_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: desc.label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        if let Some(pixel_data) = data {
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &wgpu_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                pixel_data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * desc.width),
                    rows_per_image: Some(desc.height),
                },
                size,
            );
        }

        let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());

        GpuTexture {
            id: self.next_texture_id.fetch_add(1, Ordering::Relaxed),
            wgpu_texture,
            view,
            width: desc.width,
            height: desc.height,
        }
    }

    pub fn create_sampler(&self, desc: SamplerDescriptor) -> Sampler {
        let wgpu_mag = match desc.mag_filter {
            crate::sampler::FilterMode::Nearest => wgpu::FilterMode::Nearest,
            crate::sampler::FilterMode::Linear => wgpu::FilterMode::Linear,
        };
        let wgpu_min = match desc.min_filter {
            crate::sampler::FilterMode::Nearest => wgpu::FilterMode::Nearest,
            crate::sampler::FilterMode::Linear => wgpu::FilterMode::Linear,
        };
        let wgpu_address_u = match desc.address_mode_u {
            crate::sampler::AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            crate::sampler::AddressMode::Repeat => wgpu::AddressMode::Repeat,
            crate::sampler::AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        };
        let wgpu_address_v = match desc.address_mode_v {
            crate::sampler::AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            crate::sampler::AddressMode::Repeat => wgpu::AddressMode::Repeat,
            crate::sampler::AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        };

        let wgpu_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: desc.label,
            mag_filter: wgpu_mag,
            min_filter: wgpu_min,
            address_mode_u: wgpu_address_u,
            address_mode_v: wgpu_address_v,
            ..Default::default()
        });

        Sampler { wgpu_sampler }
    }

    pub fn create_pipeline(&self, desc: PipelineDescriptor) -> Pipeline {
        let shader_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label,
            source: wgpu::ShaderSource::Wgsl(desc.shader_source.into()),
        });

        let mut all_attributes: Vec<Vec<wgpu::VertexAttribute>> = Vec::new();
        let mut vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout> = Vec::new();

        for layout in &desc.vertex_layouts {
            let mut attrs = Vec::new();
            for attr in &layout.attributes {
                let format = match attr.format {
                    crate::pipeline::VertexFormat::Float32 => wgpu::VertexFormat::Float32,
                    crate::pipeline::VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
                    crate::pipeline::VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
                    crate::pipeline::VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
                    crate::pipeline::VertexFormat::Uint32 => wgpu::VertexFormat::Uint32,
                };
                attrs.push(wgpu::VertexAttribute {
                    format,
                    offset: attr.offset as u64,
                    shader_location: attr.location,
                });
            }
            all_attributes.push(attrs);
        }

        for (i, layout) in desc.vertex_layouts.iter().enumerate() {
            let step_mode = match layout.step_mode {
                crate::pipeline::StepMode::Vertex => wgpu::VertexStepMode::Vertex,
                crate::pipeline::StepMode::Instance => wgpu::VertexStepMode::Instance,
            };
            vertex_buffer_layouts.push(wgpu::VertexBufferLayout {
                array_stride: layout.stride as u64,
                step_mode,
                attributes: &all_attributes[i],
            });
        }

        let blend_state = match desc.blend_mode {
            crate::pipeline::BlendMode::Opaque => wgpu::BlendState::REPLACE,
            crate::pipeline::BlendMode::AlphaBlend => wgpu::BlendState::ALPHA_BLENDING,
            crate::pipeline::BlendMode::Additive => wgpu::BlendState::ADDITIVE,
        };

        let bind_group_layouts_opt: Vec<Option<&wgpu::BindGroupLayout>> = desc
            .bind_group_layouts
            .iter()
            .map(|bg| Some(&bg.layout))
            .collect();

        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: desc.label,
            bind_group_layouts: &bind_group_layouts_opt,
            ..Default::default()
        });

        let buffers: Vec<Option<wgpu::VertexBufferLayout>> = vertex_buffer_layouts
            .into_iter()
            .map(Some)
            .collect();

        let wgpu_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: desc.label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_config.format,
                    blend: Some(blend_state),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let bind_group_layout: wgpu::BindGroupLayout = if desc.bind_group_layouts.is_empty() {
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: desc.label,
                entries: &[],
            })
        } else {
            desc.bind_group_layouts[0].layout.clone()
        };

        Pipeline {
            wgpu_pipeline,
            bind_group_layout,
        }
    }

    pub fn create_bind_group(&self, label: Option<&str>, entries: &[BindGroupEntry<'_>]) -> BindGroup {
        let mut layout_entries = Vec::with_capacity(entries.len());
        let mut bg_entries = Vec::with_capacity(entries.len());

        for (i, entry) in entries.iter().enumerate() {
            match entry {
                BindGroupEntry::Texture { texture, sampler } => {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    });
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: i as u32 + 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    });
                    bg_entries.push(wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    });
                    bg_entries.push(wgpu::BindGroupEntry {
                        binding: i as u32 + 1,
                        resource: wgpu::BindingResource::Sampler(&sampler.wgpu_sampler),
                    });
                }
                BindGroupEntry::UniformBuffer { buffer } => {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    });
                    bg_entries.push(wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: buffer.wgpu_buffer.as_entire_binding(),
                    });
                }
            }
        }

        let layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &layout_entries,
        });

        let wgpu_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &layout,
            entries: &bg_entries,
        });

        BindGroup {
            wgpu_bind_group,
            layout,
        }
    }

    fn get_initial_surface_config(
        surface_capabilities: &wgpu::SurfaceCapabilities,
        width: u32,
        height: u32,
        vsync: bool,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied();
        let surface_format = surface_format.unwrap_or(surface_capabilities.formats[0]);

        let present_mode = if vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            surface_capabilities
                .present_modes
                .iter()
                .find(|p| **p == wgpu::PresentMode::Mailbox)
                .copied()
                .unwrap_or(wgpu::PresentMode::AutoNoVsync)
        };

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width,
            height,
            present_mode,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        }
    }
}

impl Frame {
    pub fn clear(&mut self, color: Color) {
        self.clear_color = Some(color.into());
    }

    pub fn begin_pass(&mut self) -> RenderPass<'_> {
        self.pass_count += 1;
        let load_op = match self.clear_color.take() {
            Some(color) => wgpu::LoadOp::Clear(color),
            None => wgpu::LoadOp::Load,
        };

        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: load_op,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        RenderPass { render_pass }
    }
}
