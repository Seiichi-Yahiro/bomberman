use crate::assets::Arenas;
use amethyst::{
    assets::{Asset, Format, Handle, Loader, ProgressCounter},
    audio::Source,
    prelude::*,
    ui::FontAsset,
};
use std::collections::HashMap;
use std::marker::PhantomData;

type LoadAssetFn<A> = dyn Fn(&World, &mut ProgressCounter) -> Handle<A>;
type LoadAssetVec<A> = Vec<(&'static str, Box<LoadAssetFn<A>>)>;
type AssetHandles<A> = HashMap<&'static str, Handle<A>>;

pub struct AssetLoaders {
    fonts: LoadAssetVec<FontAsset>,
    sfx: LoadAssetVec<Source>,
    custom: LoadAssetVec<Arenas>,
}

pub struct Assets {
    pub fonts: AssetHandles<FontAsset>,
    pub sfx: AssetHandles<Source>,
    pub custom: AssetHandles<Arenas>,
}

pub struct LoadState<T> {
    phantom: PhantomData<T>,
    progress_counter: ProgressCounter,
    asset_loaders: AssetLoaders,
    assets: Option<Assets>,
}

impl<T> LoadState<T> {
    pub fn new(asset_loaders: AssetLoaders) -> LoadState<T>
    where
        T: LoadableState,
    {
        LoadState {
            phantom: PhantomData,
            progress_counter: ProgressCounter::new(),
            asset_loaders,
            assets: None,
        }
    }

    fn load_asset<A: Asset>(
        asset_loaders: &LoadAssetVec<A>,
        world: &World,
        progess: &mut ProgressCounter,
    ) -> AssetHandles<A> {
        let mut map: AssetHandles<A> = HashMap::new();

        asset_loaders.iter().for_each(|(key, load)| {
            let handle = load(world, progess);
            map.insert(key, handle);
        });

        map
    }
}

impl<T> SimpleState for LoadState<T>
where
    T: LoadableState + 'static,
{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let assets = Assets {
            fonts: Self::load_asset(&self.asset_loaders.fonts, world, &mut self.progress_counter),
            sfx: Self::load_asset(&self.asset_loaders.sfx, world, &mut self.progress_counter),
            custom: Self::load_asset(
                &self.asset_loaders.custom,
                world,
                &mut self.progress_counter,
            ),
        };

        self.assets = Some(assets);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            return Trans::Switch(T::new(self.assets.take().unwrap()));
        }

        Trans::None
    }
}

pub trait LoadableState: SimpleState + Sized {
    fn load() -> Box<LoadState<Self>>;
    fn new(assets: Assets) -> Box<Self>;
}

pub struct LoadStateBuilder {
    fonts: LoadAssetVec<FontAsset>,
    sfx: LoadAssetVec<Source>,
    custom: LoadAssetVec<Arenas>,
}

impl LoadStateBuilder {
    pub fn new() -> LoadStateBuilder {
        LoadStateBuilder {
            fonts: vec![],
            sfx: vec![],
            custom: vec![],
        }
    }

    fn with<A: Asset, F: Format<A::Data> + Clone>(
        filename: &'static str,
        format: F,
    ) -> Box<LoadAssetFn<A>> {
        let cloned_format = format.clone();

        let f = move |world: &World, progress: &mut ProgressCounter| -> Handle<A> {
            let loader = world.read_resource::<Loader>();
            loader.load(
                filename,
                cloned_format.clone(),
                progress,
                &world.read_resource(),
            )
        };

        Box::new(f)
    }

    pub fn with_font<F: Format<<FontAsset as Asset>::Data> + Clone>(
        mut self,
        key: &'static str,
        filename: &'static str,
        format: F,
    ) -> Self {
        let f = Self::with(filename, format);
        self.fonts.push((key, f));
        self
    }

    pub fn with_sfx<F: Format<<Source as Asset>::Data> + Clone>(
        mut self,
        key: &'static str,
        filename: &'static str,
        format: F,
    ) -> Self {
        let f = Self::with(filename, format);
        self.sfx.push((key, f));
        self
    }

    pub fn with_custom<F: Format<<Arenas as Asset>::Data> + Clone>(
        mut self,
        key: &'static str,
        filename: &'static str,
        format: F,
    ) -> Self {
        let f = Self::with(filename, format);
        self.custom.push((key, f));
        self
    }

    pub fn build<T>(self) -> LoadState<T>
    where
        T: LoadableState,
    {
        let asset_loaders = AssetLoaders {
            fonts: self.fonts,
            sfx: self.sfx,
            custom: self.custom,
        };
        LoadState::new(asset_loaders)
    }
}
