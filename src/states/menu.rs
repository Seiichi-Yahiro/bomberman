use crate::assets::Arenas;
use crate::assets::LoadableAssetType;
use crate::states::prelude::*;
use amethyst::{
    assets::{AssetStorage, RonFormat},
    audio::{output::Output, OggFormat, Source},
    core::Parent,
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform},
};

pub struct Menu {
    number_of_arenas: usize,
    arenas_parent_entity: Option<Entity>,
    selected_arena: usize,
    font_size: f32,
    assets: AssetHandles<LoadableAssetType>,
}

impl LoadableState<LoadableAssetType> for Menu {
    fn load() -> Box<LoadState<Self, LoadableAssetType>> {
        let load_state = LoadStateBuilder::new()
            .with::<FontAsset, TtfFormat>("font", "fonts/verdana.ttf", TtfFormat)
            .with::<Source, OggFormat>("cursor", "sfx/cursor.ogg", OggFormat)
            .with::<Arenas, RonFormat>("arenas", "arenas/arenas.ron", RonFormat)
            .build();
        Box::new(load_state)
    }

    fn new(assets: AssetHandles<LoadableAssetType>) -> Box<Self> {
        Box::new(Menu {
            assets,
            arenas_parent_entity: None,
            font_size: 30.0,
            selected_arena: 0,
            number_of_arenas: 0,
        })
    }
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.initialize_arena_selection(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            let is_down = is_key_down(&event, VirtualKeyCode::Down)
                && self.selected_arena != self.number_of_arenas - 1;
            let is_up = is_key_down(&event, VirtualKeyCode::Up) && self.selected_arena != 0;

            if is_down || is_up {
                let mut transforms = data.world.write_storage::<UiTransform>();

                if let Some(transform) = transforms.get_mut(self.arenas_parent_entity.unwrap()) {
                    let movement = if is_down { 1.0 } else { -1.0 };
                    if is_down {
                        self.selected_arena += 1
                    } else {
                        self.selected_arena -= 1
                    };
                    transform.local_y += self.font_size * movement;

                    let source = data.world.read_resource::<AssetStorage<Source>>();
                    let output = data.world.read_resource::<Output>();

                    output.play_once(
                        source
                            .get(&self.assets.get("cursor").unwrap().clone().into())
                            .unwrap(),
                        1.0,
                    );
                }
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                let arenas_assets = data.world.read_resource::<AssetStorage<Arenas>>();
                let file = &arenas_assets
                    .get(&self.assets.get("arenas").unwrap().clone().into())
                    .unwrap()
                    .arenas[self.selected_arena]
                    .file;
                println!("{}", file);
            }
        }

        Trans::None
    }
}

impl Menu {
    fn initialize_arena_selection(&mut self, world: &mut World) {
        let arena_names: Vec<String> = {
            let arenas_assets = world.read_resource::<AssetStorage<Arenas>>();
            arenas_assets
                .get(&self.assets.get("arenas").unwrap().clone().into())
                .unwrap()
                .arenas
                .iter()
                .map(|arena_data| arena_data.name.clone())
                .collect()
        };

        let parent = {
            let transform = UiTransform::new(
                "arenas_parent_transform".to_string(),
                Anchor::Middle,
                Anchor::TopMiddle,
                0.0,
                0.0,
                1.0,
                500.0, // TODO use a variable
                arena_names.len() as f32 * self.font_size,
            );

            world.create_entity().with(transform).build()
        };

        arena_names.iter().enumerate().for_each(|(index, name)| {
            let arena_name_text = UiText::new(
                self.assets.get("font").unwrap().clone().into(),
                name.clone(),
                [0.8, 0.8, 0.8, 1.0],
                self.font_size,
            );

            let arena_name_transform = UiTransform::new(
                (name.clone() + "_transform").to_string(),
                Anchor::TopMiddle,
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
                .with(Parent { entity: parent })
                .build();
        });

        self.arenas_parent_entity = Some(parent);
        self.number_of_arenas = arena_names.len();
    }
}
