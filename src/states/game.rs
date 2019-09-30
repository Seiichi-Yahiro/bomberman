use crate::assets::Arena;
use crate::assets::{AssetData, ASSET_DATAS};
use crate::states::prelude::*;
use amethyst::{
    assets::RonFormat,
    audio::output::Output,
    core::Parent,
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
};

const ARENA_KEY: &'static str = "arena";

pub struct Game {}

impl LoadableState for Game {
    type Data = String;

    fn load(data: Self::Data) -> Box<LoadState<Self>> {
        let filename = "arenas/".to_owned() + &data;
        let asset_data: AssetData<Arena, RonFormat> =
            AssetData::new(ARENA_KEY, &filename, RonFormat);

        let load_state = LoadStateBuilder::new().with(&asset_data).build();
        Box::new(load_state)
    }

    fn new() -> Box<Self> {
        Box::new(Game {})
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        with_dynamic_asset(data.world, ARENA_KEY, |arena: &Arena| {
            println!("{:?}", arena.tiles);
        });
    }
}
