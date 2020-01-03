use crate::game_states::state_manager::{GameState, Resources};
use crate::utils::asset_storage::{Asset, AssetStorage};

pub struct GameStateBuilder {
    pub build: Box<dyn FnOnce(&Resources) -> Box<dyn GameState>>,
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
        f: impl FnOnce(&Resources) -> Box<dyn GameState> + 'static,
    ) -> GameStateBuilder {
        let builder = move |resources: &Resources| {
            self.asset_loaders.into_iter().for_each(|load| {
                load(&mut *resources.asset_storage.lock().unwrap());
            });

            f(resources)
        };

        GameStateBuilder {
            build: Box::new(builder),
        }
    }
}
