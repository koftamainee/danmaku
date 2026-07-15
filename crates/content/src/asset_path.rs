use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Mod,
    Scenario,
}

impl Source {
    pub fn from_prefix(s: &str) -> Option<Self> {
        match s {
            "m" => Some(Source::Mod),
            "s" => Some(Source::Scenario),
            _ => None,
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            Source::Mod => "m",
            Source::Scenario => "s",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetPath {
    pub(crate) source: Source,
    pub(crate) source_id: Box<str>,
    pub(crate) path: Box<str>,
}

impl AssetPath {
    pub fn new(source: Source, source_id: impl Into<Box<str>>, path: impl Into<Box<str>>) -> Self {
        Self {
            source,
            source_id: source_id.into(),
            path: path.into(),
        }
    }

    pub fn parse(s: &str) -> Result<Self, AssetPathParseError> {
        let (prefix, rest) = s
            .split_once(':')
            .ok_or(AssetPathParseError::MissingSeparator)?;

        let source = Source::from_prefix(prefix)
            .ok_or(AssetPathParseError::UnknownSource(prefix.to_string()))?;

        let (source_id, path) = rest
            .split_once(':')
            .ok_or(AssetPathParseError::MissingSeparator)?;

        if source_id.is_empty() {
            return Err(AssetPathParseError::EmptySourceId);
        }
        if path.is_empty() {
            return Err(AssetPathParseError::EmptyPath);
        }

        Ok(Self::new(source, source_id, path))
    }

    pub fn source(&self) -> Source {
        self.source
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for AssetPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.source.prefix(),
            self.source_id,
            self.path
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetPathParseError {
    #[error("expected format 'source:id:path' (e.g. 's:base:assets/file.lua')")]
    MissingSeparator,
    #[error("unknown source type '{0}', expected 'm' or 's'")]
    UnknownSource(String),
    #[error("source id is empty")]
    EmptySourceId,
    #[error("path is empty")]
    EmptyPath,
}
