use std::any::Any;
use std::collections::HashMap;
use slotmap::{DefaultKey, SlotMap};
use crate::{Asset, Handle};
use crate::asset_path::AssetPath;

pub(crate) trait AnyCache: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn unload_path(&mut self, path: &AssetPath);
}

pub(crate) struct TypedCache<T: Asset> {
    pub(crate) storage: SlotMap<DefaultKey, T>,
    pub(crate) path_to_handle: HashMap<AssetPath, Handle<T>>,
    pub(crate) handle_to_path: HashMap<DefaultKey, AssetPath>,
}

impl<T: Asset> TypedCache<T> {
    pub(crate) fn new() -> Self {
        Self {
            storage: SlotMap::new(),
            path_to_handle: HashMap::new(),
            handle_to_path: HashMap::new(),
        }
    }
}

impl<T: Asset> AnyCache for TypedCache<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn unload_path(&mut self, path: &AssetPath) {
        if let Some(handle) = self.path_to_handle.remove(path) {
            self.storage.remove(handle.key);
            self.handle_to_path.remove(&handle.key);
        }
    }
}