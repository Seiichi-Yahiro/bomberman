use crate::assets::ASSET_DATAS;
use crate::states;
use crate::states::prelude::*;
use amethyst::{
    audio::output::Output,
    core::Parent,
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
};

pub struct Menu {
    number_of_arenas: usize,
    arenas_parent_entity: Option<Entity>,
    selected_arena: usize,
    font_size: f32,
}

impl LoadableState for Menu {
    type Data = ();

    fn load(_data: Self::Data) -> Box<LoadState<Self>> {
        let load_state = LoadStateBuilder::new()
            .with(&ASSET_DATAS.font_main)
            .with(&ASSET_DATAS.sfx_cursor)
            .with(&ASSET_DATAS.custom_arenas)
            .build();
        Box::new(load_state)
    }

    fn new() -> Box<Self> {
        Box::new(Menu {
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

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world
            .delete_entity(self.arenas_parent_entity.unwrap())
            .expect("Could not delete arenas selection entities!");

        let mut asset_handles = data.world.write_resource::<AssetHandles>();
        asset_handles.remove(ASSET_DATAS.font_main.name);
        asset_handles.remove(ASSET_DATAS.sfx_cursor.name);
        asset_handles.remove(ASSET_DATAS.custom_arenas.name);
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

                    with_asset(data.world, &ASSET_DATAS.sfx_cursor, |asset| {
                        let output = data.world.read_resource::<Output>();
                        output.play_once(asset, 1.0);
                    });
                }
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                return with_asset(data.world, &ASSET_DATAS.custom_arenas, |asset| {
                    let filename = asset.arenas[self.selected_arena].file.clone();
                    Trans::Switch(states::Game::load(filename))
                });
            }
        }

        Trans::None
    }
}

impl Menu {
    fn initialize_arena_selection(&mut self, world: &mut World) {
        let arena_names: Vec<String> = with_asset(world, &ASSET_DATAS.custom_arenas, |asset| {
            asset
                .arenas
                .iter()
                .map(|arena_data| arena_data.name.clone())
                .collect()
        });

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
                get_asset_handle(world, &ASSET_DATAS.font_main),
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
