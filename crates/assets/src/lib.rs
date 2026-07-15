mod atlas;
mod texture;

pub use atlas::{Atlas, AtlasError, AtlasLoadContext, Region, SpriteData, SpritesheetData};
pub use texture::{Texture, TextureData, TextureError, TextureLoadContext};

#[derive(thiserror::Error, Debug)]
#[error("lua error: {0}")]
pub struct LuaError(pub String);
