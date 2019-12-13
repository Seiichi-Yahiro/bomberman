use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

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

    pub fn load_asset_from_file<A>(&mut self, path: &Path) -> Rc<A>
    where
        A: Asset,
    {
        let asset = self
            .storage
            .entry(path.to_str().unwrap().to_string())
            .or_insert_with(|| Rc::new(A::load_from_file(path)))
            .clone();

        Rc::clone(&asset.downcast().unwrap())
    }

    pub fn release_asset(&mut self, path: &str) {
        self.storage.remove(path);
    }
}
