use content::AssetPath;
use crate::texture::{GpuTexture, Hitbox, Rect, TextureDescriptor, TextureFormat};
use crate::GraphicsDevice;

pub struct Region {
    pub src: Rect,
    pub hitbox: Option<Hitbox>,
}

pub struct Atlas {
    pub gpu: GpuTexture,
    pub regions: Vec<Region>,
}

#[derive(thiserror::Error, Debug)]
pub enum AtlasError {
    #[error("failed to load atlas texture: {0}")]
    Image(#[from] image::ImageError),
}

pub struct AtlasLoadContext<'a> {
    pub gpu: &'a GraphicsDevice,
    pub regions: Vec<Region>,
    pub label: Option<&'a str>,
}

impl content::Asset for Atlas {
    type Error = AtlasError;
    type Context<'a> = AtlasLoadContext<'a>;

    fn load(data: &[u8], _asset_path: &AssetPath, ctx: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let img = image::load_from_memory(data)?;
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

        Ok(Atlas { gpu, regions: ctx.regions })
    }
}
