use std::collections::HashMap;

use content::AssetPath;
use crate::texture::{TextureDescriptor, TextureFormat};
use crate::{GraphicsDevice, Rect, Texture};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum CollisionShape {
    #[serde(rename = "circle")]
    Circle { radius: f32 },
    #[serde(rename = "rect")]
    Rect { width: f32, height: f32 },
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SpriteRegion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub collision: Option<CollisionShape>,
}

impl SpriteRegion {
    pub fn src(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

#[derive(Debug, serde::Deserialize)]
struct AtlasJson {
    texture: String,
    sprites: Vec<SpriteEntryJson>,
}

#[derive(Debug, serde::Deserialize)]
struct SpriteEntryJson {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    collision: Option<CollisionShape>,
}

pub struct Atlas {
    texture: Texture,
    sprites: HashMap<String, SpriteRegion>,
}

#[derive(thiserror::Error, Debug)]
pub enum AtlasError {
    #[error("failed to parse atlas JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to load atlas texture: {0}")]
    Image(#[from] image::ImageError),
    #[error("texture file not found: {0}")]
    TextureNotFound(String),
}

pub struct AtlasLoadContext<'a> {
    pub graphics_device: &'a GraphicsDevice,
}

impl<'a> From<&'a GraphicsDevice> for AtlasLoadContext<'a> {
    fn from(value: &'a GraphicsDevice) -> Self {
        Self {
            graphics_device: value,
        }
    }
}

impl content::Asset for Atlas {
    type Error = AtlasError;
    type Context<'a> = AtlasLoadContext<'a>;

    fn load(data: &[u8], asset_path: &AssetPath, context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let atlas_json: AtlasJson = serde_json::from_slice(data)?;

        let json_fs_path = format!(
            "mods/{}/{}",
            asset_path.mod_id(),
            asset_path.path()
        );
        let base_dir = json_fs_path
            .rsplit_once('/')
            .map(|(dir, _)| dir)
            .unwrap_or(".");

        let texture_path = format!("{}/{}", base_dir, atlas_json.texture);
        let img = image::open(&texture_path)
            .map_err(|_| AtlasError::TextureNotFound(texture_path.clone()))?
            .to_rgba8();
        let (w, h) = img.dimensions();

        let texture = context.graphics_device.create_texture(
            TextureDescriptor {
                label: Some("Atlas texture"),
                width: w,
                height: h,
                format: TextureFormat::Rgba8UnormSrgb,
            },
            Some(&img),
        );

        let sprites = atlas_json
            .sprites
            .into_iter()
            .map(|s| {
                let region = SpriteRegion {
                    x: s.x,
                    y: s.y,
                    width: s.width,
                    height: s.height,
                    collision: s.collision,
                };
                (s.name, region)
            })
            .collect();

        Ok(Atlas {
            texture,
            sprites,
        })
    }
}

impl Atlas {
    pub fn get(&self, name: &str) -> Option<&SpriteRegion> {
        self.sprites.get(name)
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
