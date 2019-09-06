use crate::assets::Arenas;
use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::*,
};

pub struct LoadMenu {
    progress_counter: ProgressCounter,
    arenas_handle: Option<Handle<Arenas>>,
}

impl LoadMenu {
    pub fn new() -> LoadMenu {
        LoadMenu {
            progress_counter: ProgressCounter::new(),
            arenas_handle: None,
        }
    }
}

impl SimpleState for LoadMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<Arenas>>();
        let handle = loader.load(
            "arenas/arenas.ron",
            RonFormat,
            &mut self.progress_counter,
            &storage,
        );
        self.arenas_handle = Some(handle);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(Menu {
                arenas_handle: (*self.arenas_handle.as_ref().unwrap()).clone(),
            }))
        } else {
            Trans::None
        }
    }
}

pub struct Menu {
    arenas_handle: Handle<Arenas>,
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let arenas_assets = data.world.read_resource::<AssetStorage<Arenas>>();
        let arenas = &arenas_assets.get(&self.arenas_handle).unwrap().arenas;
        println!("after number of arenas {:?}", arenas);
    }
}
