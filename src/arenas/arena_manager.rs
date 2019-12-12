use crate::arenas::object_groups;
use crate::players::PlayerId;
use engine::game_state::*;
use engine::map::{TileUpdate, Map};
use graphics::math::Vec2d;
use std::collections::HashMap;

const ARENAS_FOLDER: &str = "assets/arenas/";
const TEXTURE_FOLDER: &str = "assets/textures/arena_tiles/";
const FILE_NAME: &str = "ashlands.tmx";

pub struct ArenaManager {
    map: Map,
}

impl ArenaManager {
    pub fn new() -> ArenaManager {
        let mut map = Map::new(&format!("{}{}", ARENAS_FOLDER, FILE_NAME), TEXTURE_FOLDER);
        map.update_tiles(Self::create_soft_block_tile_updates(&map));

        ArenaManager { map }
    }

    pub fn get_player_spawns(&self) -> HashMap<PlayerId, Vec2d> {
        self.map
            .object_groups
            .get(object_groups::ArenaObjectGroup::PlayerSpawns.as_str())
            .iter()
            .flat_map(|v| v.iter())
            .filter_map(|object| {
                object
                    .properties
                    .get(object_groups::PlayerSpawnsProperties::PlayerId.as_str())
                    .and_then(|property_value| match property_value {
                        tiled::PropertyValue::IntValue(player_id) => Some(player_id),
                        _ => None,
                    })
                    .map(|player_id| {
                        (
                            PlayerId::from(player_id.abs() as u32),
                            [object.x as f64, object.y as f64],
                        )
                    })
            })
            .collect()
    }

    fn create_soft_block_tile_updates(map: &Map) -> Vec<TileUpdate> {
        let should_spawn_soft_block = |soft_block: &&tiled::Object| -> bool {
            soft_block
                .properties
                .get(object_groups::SoftBlockAreasProperties::SpawnChance.as_str())
                .map(|property_value| match property_value {
                    tiled::PropertyValue::FloatValue(spawn_chance) => {
                        rand::random::<f32>() <= *spawn_chance
                    }
                    _ => false,
                })
                .unwrap_or(false)
        };

        map.object_groups
            .get(object_groups::ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(|object| {
                object
                    .properties
                    .get(object_groups::SoftBlockAreasProperties::RenderLayer.as_str())
                    .and_then(|property_value| match property_value {
                        tiled::PropertyValue::IntValue(layer_id) => Some(TileUpdate::new(
                            *layer_id as usize,
                            [object.x as u32, object.y as u32],
                            object.gid,
                        )),
                        _ => None,
                    })
            })
            .collect()
    }
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.map.draw(c, g);
    }
}
