use content::{Asset, AssetPath, Content};
use renderer::{Hitbox, Rect};

use crate::LuaError;

pub struct TextureData {
    pub file_path: String,
    pub src: Option<Rect>,
    pub hitbox: Option<Hitbox>,
}

pub struct TextureLoadContext<'a> {
    pub load_texture: &'a dyn Fn(&[u8]) -> Result<TextureData, LuaError>,
}

pub struct Texture {
    pub file_path: String,
    pub src: Option<Rect>,
    pub hitbox: Option<Hitbox>,
}

#[derive(thiserror::Error, Debug)]
pub enum TextureError {
    #[error("lua error: {0}")]
    Lua(#[from] LuaError),
    #[error("failed to read asset: {0}")]
    Io(#[from] std::io::Error),
}

impl Asset for Texture {
    type Error = TextureError;
    type Context<'a> = TextureLoadContext<'a>;

    fn load(
        content: &Content,
        asset_path: &AssetPath,
        ctx: Self::Context<'_>,
    ) -> Result<Self, Self::Error> {
        let lua_bytes = content.read_asset_bytes(asset_path)?;
        let data = (ctx.load_texture)(&lua_bytes)?;

        Ok(Texture {
            file_path: data.file_path,
            src: data.src,
            hitbox: data.hitbox,
        })
    }
}
