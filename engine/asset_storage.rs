use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;

pub trait Asset: Any {
    fn load_from_file(path: &Path) -> Self
    where
        Self: Sized;
}

pub struct AssetStorage {
    storage: HashMap<String, Rc<dyn Any>>,
}

impl AssetStorage {
    pub fn new() -> AssetStorage {
        AssetStorage {
            storage: HashMap::new(),
        }
    }

    pub fn load_asset_from_file<A: Asset>(&mut self, path: &Path, id: &str) {
        let asset = A::load_from_file(path);
        self.storage.insert(id.to_string(), Rc::new(asset));
    }

    pub fn release_asset(&mut self, id: &str) {
        self.storage.remove(id);
    }

    pub fn get_asset<A: Asset>(&self, id: &str) -> Rc<A> {
        self.storage
            .get(id)
            .map(|asset| Rc::clone(asset).downcast::<A>().unwrap())
            .unwrap()
    }
}
