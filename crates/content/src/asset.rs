use crate::AssetPath;

pub trait Asset: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    type Context<'a>;
    fn load(data: &[u8], asset_path: &AssetPath, context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}