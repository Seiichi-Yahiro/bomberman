use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub trait Asset: Any + Send + Sync {
    fn load_from_file(path: &Path) -> Self
    where
        Self: Sized;
}

#[derive(Default)]
pub struct AssetStorage {
    storage: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl AssetStorage {
    pub fn new() -> AssetStorage {
        AssetStorage {
            storage: HashMap::new(),
        }
    }

    pub fn load_asset_from_file<A: Asset>(&mut self, path: &Path, id: &str) {
        let asset = A::load_from_file(path);
        self.storage.insert(id.to_string(), Arc::new(asset));
    }

    pub fn release_asset(&mut self, id: &str) {
        self.storage.remove(id);
    }

    pub fn get_asset<A: Asset>(&self, id: &str) -> Arc<A> {
        self.storage
            .get(id)
            .map(|asset| Arc::clone(asset).downcast::<A>().unwrap())
            .unwrap()
    }
}
