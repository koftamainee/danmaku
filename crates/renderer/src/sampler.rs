#[derive(Clone, Copy)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Clone, Copy)]
pub enum AddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

pub struct SamplerDescriptor<'a> {
    pub label: Option<&'a str>,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
}

pub struct Sampler {
    pub(crate) wgpu_sampler: wgpu::Sampler,
}
