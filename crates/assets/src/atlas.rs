use content::{Asset, AssetPath, Content};
use renderer::{Hitbox, Rect};

use crate::LuaError;

pub struct SpriteData {
    pub name: String,
    pub src: Rect,
    pub hitbox: Option<Hitbox>,
}

pub struct SpritesheetData {
    pub texture_path: String,
    pub sprites: Vec<SpriteData>,
}

pub struct AtlasLoadContext<'a> {
    pub load_spritesheet: &'a dyn Fn(&[u8]) -> Result<SpritesheetData, LuaError>,
}

pub struct Atlas {
    pub texture_path: String,
    pub regions: Vec<Region>,
}

pub struct Region {
    pub src: Rect,
    pub hitbox: Option<Hitbox>,
}

#[derive(thiserror::Error, Debug)]
pub enum AtlasError {
    #[error("lua error: {0}")]
    Lua(#[from] LuaError),
    #[error("failed to read asset: {0}")]
    Io(#[from] std::io::Error),
}

impl Asset for Atlas {
    type Error = AtlasError;
    type Context<'a> = AtlasLoadContext<'a>;

    fn load(
        content: &Content,
        asset_path: &AssetPath,
        ctx: Self::Context<'_>,
    ) -> Result<Self, Self::Error> {
        let lua_bytes = content.read_asset_bytes(asset_path)?;
        let data = (ctx.load_spritesheet)(&lua_bytes)?;

        let regions = data
            .sprites
            .into_iter()
            .map(|s| Region {
                src: s.src,
                hitbox: s.hitbox,
            })
            .collect();

        Ok(Atlas {
            texture_path: data.texture_path,
            regions,
        })
    }
}
