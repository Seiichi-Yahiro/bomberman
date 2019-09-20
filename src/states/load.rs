use amethyst::{
    assets::{Asset, Format, Handle, Loader, ProgressCounter},
    prelude::*,
};
use std::collections::HashMap;
use std::marker::PhantomData;

type AssetLoaders<E> = HashMap<&'static str, Box<dyn Fn(&World, &mut ProgressCounter) -> E>>;
pub type AssetHandles<E> = HashMap<&'static str, E>;

pub struct LoadState<S, E>
where
    S: LoadableState<E>,
    E: Clone,
{
    phantom: PhantomData<S>,
    progress_counter: ProgressCounter,
    asset_loaders: AssetLoaders<E>,
    assets: AssetHandles<E>,
}

impl<S, E> LoadState<S, E>
where
    S: LoadableState<E>,
    E: Clone,
{
    pub fn new(asset_loaders: AssetLoaders<E>) -> LoadState<S, E>
    where
        S: LoadableState<E>,
        E: Clone,
    {
        LoadState {
            phantom: PhantomData,
            progress_counter: ProgressCounter::new(),
            asset_loaders,
            assets: HashMap::new(),
        }
    }
}

impl<S, E> SimpleState for LoadState<S, E>
where
    S: LoadableState<E> + 'static,
    E: Clone,
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

pub trait LoadableState<E>: SimpleState + Sized
where
    E: Clone,
{
    fn load() -> Box<LoadState<Self, E>>;
    fn new(assets: AssetHandles<E>) -> Box<Self>;
}

pub struct LoadStateBuilder<E>
where
    E: Clone,
{
    asset_loaders: AssetLoaders<E>,
}

impl<E> LoadStateBuilder<E>
where
    E: Clone,
{
    pub fn new() -> LoadStateBuilder<E> {
        LoadStateBuilder {
            asset_loaders: HashMap::new(),
        }
    }

    pub fn with<A, F>(mut self, key: &'static str, filename: &'static str, format: F) -> Self
    where
        A: Asset,
        F: Format<A::Data> + Clone,
        E: From<Handle<A>> + Into<Handle<A>> + Clone,
    {
        let f = move |world: &World, progress: &mut ProgressCounter| -> E {
            let loader = world.read_resource::<Loader>();
            let handle = loader.load(filename, format.clone(), progress, &world.read_resource());
            E::from(handle)
        };

        self.asset_loaders.insert(key, Box::new(f));

        self
    }

    pub fn build<S>(self) -> LoadState<S, E>
    where
        S: LoadableState<E>,
        E: Clone,
    {
        LoadState::new(self.asset_loaders)
    }
}
