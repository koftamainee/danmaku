use crate::asset_path::AssetPath;
use crate::cache::{AnyCache, TypedCache};
use crate::{Asset, Handle};
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Content {
    caches: HashMap<TypeId, Box<dyn AnyCache>>,
}

impl Content {
    pub fn new() -> Self {
        Self {
            caches: HashMap::new(),
        }
    }

    fn cache_mut<T: Asset>(&mut self) -> &mut TypedCache<T> {
        self.caches
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(TypedCache::<T>::new()))
            .as_any_mut()
            .downcast_mut::<TypedCache<T>>()
            .expect("TypeId lookup should produce correct downcast")
    }

    fn cache<T: Asset>(&self) -> Option<&TypedCache<T>> {
        self.caches
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<TypedCache<T>>()
    }

    pub fn load<T: Asset>(
        &mut self,
        path: &AssetPath,
        context: T::Context<'_>,
    ) -> Result<Handle<T>, LoadError<T::Error>> {
        if let Some(&handle) = self.cache_mut::<T>().path_to_handle.get(path) {
            return Ok(handle);
        }

        let data = read_asset_bytes(path)?;
        let asset = T::load(&data, path, context).map_err(LoadError::Asset)?;

        let cache = self.cache_mut::<T>();
        let key = cache.storage.insert(asset);

        let handle = Handle {
            key,
            _marker: PhantomData,
        };

        cache.path_to_handle.insert(path.clone(), handle);
        cache.handle_to_path.insert(key, path.clone());

        Ok(handle)
    }

    pub fn get<T: Asset>(&self, handle: Handle<T>) -> Option<&T> {
        self.cache::<T>()?.storage.get(handle.key)
    }

    pub fn get_mut<T: Asset>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.caches
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<TypedCache<T>>()?
            .storage
            .get_mut(handle.key)
    }

    pub fn unload_path(&mut self, path: &AssetPath) {
        for cache in self.caches.values_mut() {
            cache.unload_path(path);
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoadError<E: std::error::Error + 'static> {
    #[error("failed to read asset file")]
    Io(#[from] std::io::Error),
    #[error("failed to parse asset")]
    Asset(#[source] E),
}

fn read_asset_bytes(path: &AssetPath) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(format!("mods/{}/{}", path.mod_id, path.path))
}
