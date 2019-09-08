use crate::assets::Arenas;
use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
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
            Trans::Switch(Box::new(Menu::new(
                (*self.arenas_handle.as_ref().unwrap()).clone(),
            )))
        } else {
            Trans::None
        }
    }
}

pub struct Menu {
    arenas_handle: Handle<Arenas>,
    arenas_entities: Vec<Entity>,
    selected_arena: usize,
    font_size: f32,
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.arenas_entities = self.initialize_arena_selection(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            let is_down = is_key_down(&event, VirtualKeyCode::Down)
                && self.selected_arena != self.arenas_entities.len() - 1;
            let is_up = is_key_down(&event, VirtualKeyCode::Up) && self.selected_arena != 0;

            if is_down || is_up {
                let mut transforms = data.world.write_storage::<UiTransform>();
                let movement = if is_down { 1.0 } else { -1.0 };

                for (index, entity) in self.arenas_entities.iter().enumerate() {
                    if let Some(transform) = transforms.get_mut(*entity) {
                        let new_y = transform.local_y + self.font_size * movement;

                        if new_y == 0.0 {
                            self.selected_arena = index;
                        }

                        transform.local_y = new_y;
                    };
                }
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                let arenas_assets = data.world.read_resource::<AssetStorage<Arenas>>();
                let file = &arenas_assets.get(&self.arenas_handle).unwrap().arenas
                    [self.selected_arena]
                    .file;
                println!("{}", file);
            }
        }

        Trans::None
    }
}

impl Menu {
    pub fn new(arenas_handle: Handle<Arenas>) -> Menu {
        Menu {
            arenas_handle,
            arenas_entities: vec![],
            font_size: 30.0,
            selected_arena: 0,
        }
    }

    fn initialize_arena_selection(&self, world: &mut World) -> Vec<Entity> {
        let font = world.read_resource::<Loader>().load(
            "fonts/verdana.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let arena_names: Vec<String> = {
            let arenas_assets = world.read_resource::<AssetStorage<Arenas>>();
            arenas_assets
                .get(&self.arenas_handle)
                .unwrap()
                .arenas
                .iter()
                .map(|arena_data| arena_data.name.clone())
                .collect()
        };

        arena_names
            .iter()
            .enumerate()
            .map(|(index, name)| {
                let arena_name_text = UiText::new(
                    font.clone(),
                    name.clone(),
                    [0.8, 0.8, 0.8, 1.0],
                    self.font_size,
                );

                let arena_name_transform = UiTransform::new(
                    (name.clone() + "_transform").to_string(),
                    Anchor::Middle,
                    Anchor::Middle,
                    0.0,
                    -self.font_size * index as f32,
                    1.0,
                    500.0, // TODO use a variable
                    self.font_size,
                );

                world
                    .create_entity()
                    .with(arena_name_transform)
                    .with(arena_name_text)
                    .build()
            })
            .collect()
    }
}
