use crate::asset_storage::{Asset, AssetStorage};
use crate::state_manager::GameState;

pub struct GameStateBuilder {
    pub build: Box<dyn FnOnce(&mut AssetStorage) -> Box<dyn GameState>>,
}

#[derive(Default)]
pub struct GameStateBuilderBuilder {
    asset_loaders: Vec<Box<dyn FnOnce(&mut AssetStorage)>>,
}

impl GameStateBuilderBuilder {
    pub fn new() -> GameStateBuilderBuilder {
        Self::default()
    }

    pub fn load_asset<A: Asset>(mut self, path: &str, id: &str) -> Self {
        let path = path.to_string();
        let id = id.to_string();
        let f = move |asset_storage: &mut AssetStorage| {
            asset_storage.load_asset_from_file::<A>(std::path::Path::new(&path), &id);
        };

        self.asset_loaders.push(Box::new(f));
        self
    }

    pub fn load_asset_with(mut self, f: impl FnOnce(&mut AssetStorage) + 'static) -> Self {
        self.asset_loaders.push(Box::new(f));
        self
    }

    pub fn build(
        self,
        f: impl FnOnce(&AssetStorage) -> Box<dyn GameState> + 'static,
    ) -> GameStateBuilder {
        let builder = move |asset_storage: &mut AssetStorage| {
            self.asset_loaders.into_iter().for_each(|load| {
                load(asset_storage);
            });

            f(asset_storage)
        };

        GameStateBuilder {
            build: Box::new(builder),
        }
    }
}
