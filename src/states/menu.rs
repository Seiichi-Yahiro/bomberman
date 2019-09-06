use crate::assets::Arenas;
use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
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
        let world = data.world;
        initialize_arena_selection(world, &self.arenas_handle);
    }
}

fn initialize_arena_selection(world: &mut World, arenas_handle: &Handle<Arenas>) {
    let font = world.read_resource::<Loader>().load(
        "fonts/verdana.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let arena_names: Vec<String> = {
        let arenas_assets = world.read_resource::<AssetStorage<Arenas>>();
        arenas_assets
            .get(arenas_handle)
            .unwrap()
            .arenas
            .iter()
            .map(|arena_data| arena_data.name.clone())
            .collect()
    };

    for (index, name) in arena_names.iter().enumerate() {
        let size = 30.0;

        let arena_name_text = UiText::new(font.clone(), name.clone(), [0.8, 0.8, 0.8, 1.0], size);

        let arena_name_transform = UiTransform::new(
            (name.clone() + "_transform").to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            -size * index as f32,
            1.0,
            500.0, // TODO use a variable
            size,
        );

        world
            .create_entity()
            .with(arena_name_transform)
            .with(arena_name_text)
            .build();
    }
}
