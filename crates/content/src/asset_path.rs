use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetPath {
    pub(crate) mod_id: ModId,
    pub(crate) path: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModId(Box<str>);

impl std::fmt::Display for ModId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AssetPath {
    pub fn new(mod_id: impl Into<Box<str>>, path: impl Into<Box<str>>) -> Self {
        Self {
            mod_id: ModId(mod_id.into()),
            path: path.into(),
        }
    }
    pub fn parse(s: &str) -> Result<Self, AssetPathParseError> {
        let (mod_id, path) = s
            .split_once(':')
            .ok_or(AssetPathParseError::MissingNamespace)?;

        if mod_id.is_empty() {
            return Err(AssetPathParseError::EmptyModId);
        }
        if path.is_empty() {
            return Err(AssetPathParseError::EmptyPath);
        }

        Ok(Self::new(mod_id, path))
    }

    pub fn mod_id(&self) -> &str {
        &self.mod_id.0
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn with_mod(&self, mod_id: impl Into<Box<str>>) -> Self {
        Self {
            mod_id: ModId(mod_id.into()),
            path: self.path.clone(),
        }
    }
}

impl fmt::Display for AssetPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.mod_id.0, self.path)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetPathParseError {
    #[error("asset path is missing a mod namespace (expected 'mod_id:path')")]
    MissingNamespace,
    #[error("asset path has an empty mod id")]
    EmptyModId,
    #[error("asset path has an empty path")]
    EmptyPath,
}
