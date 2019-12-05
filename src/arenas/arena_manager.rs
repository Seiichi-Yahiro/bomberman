use crate::arenas::object_groups;
use crate::players::PlayerId;
use engine::game_state::*;
use engine::map::{Map, TileUpdate};
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

    /*pub fn get_player_spawns(&self) -> HashMap<PlayerId, Vec2d> {
        self.tile_map
            .object_groups
            .iter()
            .filter(|group| group.name == object_groups::ArenaObjectGroup::PlayerSpawns.as_str())
            .flat_map(|group| &group.objects)
            .map(|object| {
                if let tiled::PropertyValue::IntValue(player_id) =
                    object.properties[object_groups::PlayerSpawnsProperties::PlayerId.as_str()]
                {
                    return (
                        PlayerId::from(player_id.abs() as u32),
                        [
                            object.x as f64,
                            object.y as f64 - self.tile_map.tile_height as f64, // subtract tile_height as tiled origin is bottom left
                        ],
                    );
                }

                panic!("No player spawns found!");
            })
            .collect()
    }*/

    fn create_soft_block_tile_updates(map: &Map) -> Vec<TileUpdate> {
        let should_spawn_soft_block = |soft_block: &&tiled::Object| -> bool {
            match soft_block
                .properties
                .get(object_groups::SoftBlockAreasProperties::SpawnChance.as_str())
            {
                Some(tiled::PropertyValue::FloatValue(spawn_chance)) => {
                    rand::random::<f32>() <= *spawn_chance
                }
                _ => false,
            }
        };

        map.object_groups
            .get(object_groups::ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(|object| {
                match object
                    .properties
                    .get(object_groups::SoftBlockAreasProperties::RenderLayer.as_str())
                {
                    Some(tiled::PropertyValue::IntValue(layer_id)) => Some(TileUpdate::new(
                        *layer_id as usize,
                        [object.x as u32, object.y as u32],
                        object.gid,
                    )),
                    _ => None,
                }
            })
            .collect()
    }
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.map.draw(c, g);
    }
}
