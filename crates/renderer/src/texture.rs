use content::{Asset, AssetPath, Content};

pub enum TextureFormat {
    Rgba8UnormSrgb,
    Rgba8Unorm,
}

pub struct TextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

pub struct GpuTextureLoadContext<'a> {
    pub gpu: &'a crate::GraphicsDevice,
    pub label: Option<&'a str>,
}

#[derive(thiserror::Error, Debug)]
pub enum GpuTextureError {
    #[error("failed to read asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to decode image: {0}")]
    Image(#[from] image::ImageError),
}

pub struct GpuTexture {
    pub(crate) wgpu_texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Asset for GpuTexture {
    type Error = GpuTextureError;
    type Context<'a> = GpuTextureLoadContext<'a>;

    fn load(
        content: &Content,
        asset_path: &AssetPath,
        ctx: Self::Context<'_>,
    ) -> Result<Self, Self::Error> {
        let bytes = content.read_asset_bytes(asset_path)?;
        let img = image::load_from_memory(&bytes)?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let gpu = ctx.gpu.create_texture(
            TextureDescriptor {
                label: ctx.label,
                width: w,
                height: h,
                format: TextureFormat::Rgba8UnormSrgb,
            },
            Some(&rgba),
        );
        Ok(gpu)
    }
}

impl GpuTexture {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn global_id(&self) -> u64 {
        std::ptr::from_ref(&self.wgpu_texture) as u64
    }

    pub fn id(&self) -> usize {
        self as *const Self as usize
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Hitbox {
    Circle {
        radius: f32,
        origin: [f32; 2],
    },
    Rect {
        width: f32,
        height: f32,
        origin: [f32; 2],
    },
}
