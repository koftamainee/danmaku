use content::AssetPath;
use crate::GraphicsDevice;

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

pub struct GpuTexture {
    pub(crate) wgpu_texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) width: u32,
    pub(crate) height: u32,
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

#[derive(thiserror::Error, Debug)]
pub enum TextureLoadError {
    #[error("failed to decode image: {0}")]
    ImageDecode(#[from] image::ImageError),
}

pub struct TextureLoadContext<'a> {
    pub graphics_device: &'a GraphicsDevice,
    pub label: Option<&'a str>,
}

impl<'a> From<&'a GraphicsDevice> for TextureLoadContext<'a> {
    fn from(value: &'a GraphicsDevice) -> Self {
        Self {
            graphics_device: value,
            label: None,
        }
    }
}

impl content::Asset for GpuTexture {
    type Error = TextureLoadError;
    type Context<'a> = TextureLoadContext<'a>;

    fn load(data: &[u8], _asset_path: &AssetPath, context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let img = image::load_from_memory(data)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let texture = context.graphics_device.create_texture(
            TextureDescriptor {
                label: context.label,
                width,
                height,
                format: TextureFormat::Rgba8UnormSrgb,
            },
            Some(&rgba),
        );

        Ok(texture)
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
        Self { x, y, width, height }
    }
}

#[derive(Clone, Copy)]
pub enum Hitbox {
    Circle { radius: f32, origin: [f32; 2] },
    Rect { width: f32, height: f32, origin: [f32; 2] },
}
