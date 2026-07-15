use crate::asset_path::{AssetPath, Source};
use crate::cache::{AnyCache, TypedCache};
use crate::{Asset, Handle};
use std::any::TypeId;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::io;
use std::marker::PhantomData;
use std::path::PathBuf;

struct AssetReader {
    scenarios_dir: PathBuf,
    mods_dir: PathBuf,
}

impl AssetReader {
    fn new() -> Self {
        Self {
            scenarios_dir: PathBuf::from("scenarios"),
            mods_dir: PathBuf::from("mods"),
        }
    }

    fn resolve(&self, path: &AssetPath) -> PathBuf {
        let base = match path.source() {
            Source::Scenario => &self.scenarios_dir,
            Source::Mod => &self.mods_dir,
        };
        base.join(path.source_id()).join(path.path())
    }
}

pub struct Content {
    reader: AssetReader,
    caches: RefCell<HashMap<TypeId, Box<dyn AnyCache>>>,
}

impl Content {
    pub fn new() -> Self {
        Self {
            reader: AssetReader::new(),
            caches: RefCell::new(HashMap::new()),
        }
    }

    pub fn read_asset_bytes(&self, path: &AssetPath) -> io::Result<Vec<u8>> {
        let resolved = self.reader.resolve(path);
        std::fs::read(resolved)
    }

    pub fn load<T: Asset>(
        &self,
        asset_path: &AssetPath,
        context: T::Context<'_>,
    ) -> Result<Handle<T>, LoadError<T::Error>> {
        {
            let caches = self.caches.borrow();
            if let Some(handle) = caches
                .get(&TypeId::of::<T>())
                .and_then(|c| c.as_any().downcast_ref::<TypedCache<T>>())
                .and_then(|c| c.path_to_handle.get(asset_path))
            {
                return Ok(*handle);
            }
        }

        let asset = T::load(self, asset_path, context).map_err(LoadError::Asset)?;

        let mut caches = self.caches.borrow_mut();
        let cache = caches
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(TypedCache::<T>::new()))
            .as_any_mut()
            .downcast_mut::<TypedCache<T>>()
            .expect("TypeId lookup should produce correct downcast");

        let key = cache.storage.insert(asset);
        let handle = Handle {
            key,
            _marker: PhantomData,
        };

        cache.path_to_handle.insert(asset_path.clone(), handle);
        cache.handle_to_path.insert(key, asset_path.clone());

        Ok(handle)
    }

    pub fn get<T: Asset>(&self, handle: Handle<T>) -> Ref<'_, T> {
        let caches = self.caches.borrow();
        Ref::map(caches, |caches| {
            caches
                .get(&TypeId::of::<T>())
                .expect("asset type not registered")
                .as_any()
                .downcast_ref::<TypedCache<T>>()
                .expect("type downcast failed")
                .storage
                .get(handle.key)
                .expect("invalid handle")
        })
    }

    pub fn get_mut<T: Asset>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.caches
            .get_mut()
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<TypedCache<T>>()?
            .storage
            .get_mut(handle.key)
    }

    pub fn unload_path(&mut self, path: &AssetPath) {
        for cache in self.caches.get_mut().values_mut() {
            cache.unload_path(path);
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoadError<E: std::error::Error + 'static> {
    #[error("failed to read asset file")]
    Io(#[from] io::Error),
    #[error("failed to parse asset")]
    Asset(#[source] E),
}
