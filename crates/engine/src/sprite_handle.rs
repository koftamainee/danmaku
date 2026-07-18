use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub enum SpriteHandle {
    Texture { key: slotmap::DefaultKey },
    Atlas { key: slotmap::DefaultKey, index: u16 },
}

impl SpriteHandle {
    pub fn standalone(key: slotmap::DefaultKey) -> Self {
        Self::Texture { key }
    }

    pub fn from_atlas(key: slotmap::DefaultKey, index: u16) -> Self {
        Self::Atlas { key, index }
    }

    pub fn sheet_key(&self) -> Option<slotmap::DefaultKey> {
        match self {
            SpriteHandle::Texture { key } => Some(*key),
            SpriteHandle::Atlas { key, .. } => Some(*key),
        }
    }

    pub fn sprite_index(&self) -> Option<u16> {
        match self {
            SpriteHandle::Texture { .. } => None,
            SpriteHandle::Atlas { index, .. } => Some(*index),
        }
    }
}

impl PartialEq for SpriteHandle {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Texture { key: a }, Self::Texture { key: b }) => a == b,
            (Self::Atlas { key: ka, index: ia }, Self::Atlas { key: kb, index: ib }) => {
                ka == kb && ia == ib
            }
            _ => false,
        }
    }
}

impl Eq for SpriteHandle {}

impl Hash for SpriteHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            SpriteHandle::Texture { key } => key.hash(state),
            SpriteHandle::Atlas { key, index } => {
                key.hash(state);
                index.hash(state);
            }
        }
    }
}
