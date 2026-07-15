use crate::{AssetPath, Content};

pub trait Asset: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    type Context<'a>;

    fn load(
        content: &Content,
        asset_path: &AssetPath,
        context: Self::Context<'_>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
