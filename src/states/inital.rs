use crate::states::prelude::*;
use crate::states::Menu;
use amethyst::prelude::*;
use std::collections::HashMap;

pub struct InitialState;

impl SimpleState for InitialState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let asset_handles: AssetHandles = HashMap::new();
        data.world.add_resource(asset_handles);
        Trans::Switch(Menu::load())
    }
}
