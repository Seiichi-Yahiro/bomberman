use crate::assets::{AssetData, LoadableAssetType};
use amethyst::{
    assets::{Asset, Format, Handle, Loader, ProgressCounter},
    prelude::*,
};
use std::collections::HashMap;
use std::marker::PhantomData;

type AssetLoaders =
    HashMap<&'static str, Box<dyn Fn(&World, &mut ProgressCounter) -> LoadableAssetType>>;
pub type AssetHandles = HashMap<&'static str, LoadableAssetType>;

pub struct LoadState<S>
where
    S: LoadableState,
{
    phantom: PhantomData<S>,
    progress_counter: ProgressCounter,
    asset_loaders: AssetLoaders,
    assets: AssetHandles,
}

impl<S> LoadState<S>
where
    S: LoadableState,
{
    pub fn new(asset_loaders: AssetLoaders) -> LoadState<S>
    where
        S: LoadableState,
    {
        LoadState {
            phantom: PhantomData,
            progress_counter: ProgressCounter::new(),
            asset_loaders,
            assets: HashMap::new(),
        }
    }
}

impl<S> SimpleState for LoadState<S>
where
    S: LoadableState + 'static,
{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        for (key, load) in self.asset_loaders.iter() {
            let e_handle = load(world, &mut self.progress_counter);
            self.assets.insert(key, e_handle);
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            return Trans::Switch(S::new(self.assets.clone()));
        }

        Trans::None
    }
}

pub trait LoadableState: SimpleState + Sized {
    fn load() -> Box<LoadState<Self>>;
    fn new(assets: AssetHandles) -> Box<Self>;
}

pub struct LoadStateBuilder {
    asset_loaders: AssetLoaders,
}

impl LoadStateBuilder {
    pub fn new() -> LoadStateBuilder {
        LoadStateBuilder {
            asset_loaders: HashMap::new(),
        }
    }

    pub fn with<A, F>(mut self, asset_data: &'static AssetData<A, F>) -> Self
    where
        A: Asset,
        F: Format<A::Data> + Clone,
        LoadableAssetType: From<Handle<A>> + Into<Handle<A>> + Clone,
    {
        let f = move |world: &World, progress: &mut ProgressCounter| -> LoadableAssetType {
            let loader = world.read_resource::<Loader>();
            let handle = loader.load(
                asset_data.filename,
                asset_data.format.clone(),
                progress,
                &world.read_resource(),
            );
            LoadableAssetType::from(handle)
        };

        self.asset_loaders.insert(asset_data.name, Box::new(f));

        self
    }

    pub fn build<S>(self) -> LoadState<S>
    where
        S: LoadableState,
    {
        LoadState::new(self.asset_loaders)
    }
}
