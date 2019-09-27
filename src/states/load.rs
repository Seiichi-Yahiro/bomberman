use crate::assets::{AssetData, LoadableAssetType};
use amethyst::{
    assets::{Asset, AssetStorage, Format, Handle, Loader, ProgressCounter},
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
        }
    }
}

impl<S> SimpleState for LoadState<S>
where
    S: LoadableState + 'static,
{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut asset_handles = world.write_resource::<AssetHandles>();

        for (key, load) in self.asset_loaders.iter() {
            let handle = load(world, &mut self.progress_counter);
            asset_handles.insert(key, handle);
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            return Trans::Switch(S::new());
        }

        Trans::None
    }
}

pub trait LoadableState: SimpleState + Sized {
    fn load() -> Box<LoadState<Self>>;
    fn new() -> Box<Self>;
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
        LoadableAssetType: From<Handle<A>>,
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

pub fn get_asset_handle<A, F>(world: &World, asset_data: &'static AssetData<A, F>) -> Handle<A>
where
    A: Asset,
    F: Format<A::Data>,
    LoadableAssetType: Into<Handle<A>> + Clone,
{
    let asset_handles = world.read_resource::<AssetHandles>();
    asset_handles.get(asset_data.name).unwrap().clone().into()
}

pub fn with_asset<A, F, R>(
    world: &World,
    asset_data: &'static AssetData<A, F>,
    f: impl Fn(&A) -> R,
) -> R
where
    A: Asset,
    F: Format<A::Data>,
    LoadableAssetType: Into<Handle<A>> + Clone,
{
    let handle = get_asset_handle(world, asset_data);
    let assets = world.read_resource::<AssetStorage<A>>();
    let asset = assets.get(&handle).unwrap();
    f(asset)
}
