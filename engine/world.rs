use crate::asset_storage::AssetStorage;
use crate::map::Map;
use crate::traits::game_loop_event::Updatable;

pub struct World {
    pub asset_storage: AssetStorage,
    map: Option<Map>,
}

impl World {
    pub fn new() -> World {
        World {
            asset_storage: AssetStorage::new(),
            map: None,
        }
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = Some(map)
    }

    pub fn get_map(&self) -> &Map {
        self.map.as_ref().unwrap()
    }

    pub fn get_mut_map(&mut self) -> &mut Map {
        self.map.as_mut().unwrap()
    }
}
