use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub trait Asset {
    fn load_from_file<E>(path: &str) -> Result<Self, E>
    where
        Self: Sized,
        E: Error;
}

pub struct AssetStorage {
    storage: HashMap<&'static str, Rc<dyn Asset>>,
}

impl AssetStorage {
    pub fn new() -> AssetStorage {
        AssetStorage {
            storage: HashMap::new(),
        }
    }

    pub fn load_asset_from_file<A: Asset>(&mut self, path: &str) -> Rc<A> {
        let asset = self
            .storage
            .entry(path)
            .or_insert_with(|| A::load_from_file(path).unwrap());

        Rc::clone(asset)
    }

    pub fn release_asset(&mut self, path: &str) {
        self.storage.remove(path);
    }
}
