use content::AssetPath;
use crate::texture::{GpuTexture, Hitbox, Rect, TextureDescriptor, TextureFormat, TextureLoadError};
use crate::GraphicsDevice;

pub struct Texture {
    pub gpu: GpuTexture,
    pub rect: Option<Rect>,
    pub hitbox: Option<Hitbox>,
}

pub struct TextureLoadContext<'a> {
    pub gpu: &'a GraphicsDevice,
    pub rect: Option<Rect>,
    pub hitbox: Option<Hitbox>,
    pub label: Option<&'a str>,
}

impl content::Asset for Texture {
    type Error = TextureLoadError;
    type Context<'a> = TextureLoadContext<'a>;

    fn load(data: &[u8], _asset_path: &AssetPath, ctx: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let img = image::load_from_memory(data)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let gpu = ctx.gpu.create_texture(
            TextureDescriptor {
                label: ctx.label,
                width,
                height,
                format: TextureFormat::Rgba8UnormSrgb,
            },
            Some(&rgba),
        );

        Ok(Texture {
            gpu,
            rect: ctx.rect,
            hitbox: ctx.hitbox,
        })
    }
}
