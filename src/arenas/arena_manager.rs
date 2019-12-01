use crate::arenas::object_groups;
use crate::players::PlayerId;
use engine::game_state::*;
use engine::texture::*;
use graphics::math::Vec2d;
use graphics::Transformed;
use sprite::Sprite;
use std::collections::HashMap;
use std::path::Path;

const ARENAS_FOLDER: &str = "assets/arenas/";
const TEXTURE_FOLDER: &str = "assets/textures/arena_tiles/";
const FILE_NAME: &str = "ashlands.tmx";

struct ArenaTile(pub u32, pub u32, pub u32); // x, y, tile_id
type SoftBlockAreas<'a> = HashMap<[u32; 3], &'a tiled::Object>; // x, y, layer_id

pub struct ArenaManager {
    tile_map: tiled::Map,
    arena_layer_tiles: Vec<Vec<ArenaTile>>,
    textures: TextureHolder,
}

impl ArenaManager {
    pub fn new() -> ArenaManager {
        let tile_map = {
            let path = format!("{}{}", ARENAS_FOLDER, FILE_NAME);
            tiled::parse_file(&Path::new(&path)).unwrap()
        };

        ArenaManager {
            arena_layer_tiles: Self::init_arena_tiles(&tile_map),
            textures: TextureHolder::from_map(&tile_map, TEXTURE_FOLDER),
            tile_map,
        }
    }

    fn init_arena_tiles(tile_map: &tiled::Map) -> Vec<Vec<ArenaTile>> {
        let soft_block_areas = Self::get_soft_block_areas(tile_map);
        let (tile_width, tile_height) = (tile_map.tile_width, tile_map.tile_height);

        tile_map
            .layers
            .iter()
            .enumerate()
            .map(|(layer_id, layer)| {
                layer
                    .tiles
                    .iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .map(|(x, &tile)| {
                                let x = x as u32 * tile_width;
                                let y = y as u32 * tile_height;

                                // subtract 1 from the tile id as tiled counts from 1 instead of 0
                                if let Some(soft_block) =
                                    soft_block_areas.get(&[x, y, layer_id as u32])
                                {
                                    if Self::should_spawn_soft_block(soft_block) {
                                        return ArenaTile(x, y, soft_block.gid);
                                    }
                                }

                                ArenaTile(x, y, tile)
                            })
                            .collect::<Vec<ArenaTile>>()
                    })
                    .collect()
            })
            .collect()
    }

    pub fn get_player_spawns(&self) -> HashMap<PlayerId, Vec2d> {
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
    }

    fn get_soft_block_areas(tile_map: &tiled::Map) -> SoftBlockAreas {
        tile_map
            .object_groups
            .iter()
            .filter(|group| group.name == object_groups::ArenaObjectGroup::SoftBlockAreas.as_str())
            .flat_map(|group| &group.objects)
            .filter_map(|object| {
                if let tiled::PropertyValue::IntValue(layer_id) =
                    object.properties[object_groups::SoftBlockAreasProperties::RenderLayer.as_str()]
                {
                    Some((
                        [
                            object.x as u32,
                            object.y as u32 - tile_map.tile_height,
                            layer_id as u32,
                        ], // subtract tile height as object tiles have their origin in the bottom left corner
                        object,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    fn should_spawn_soft_block(soft_block: &tiled::Object) -> bool {
        if let tiled::PropertyValue::FloatValue(spawn_chance) =
            soft_block.properties[object_groups::SoftBlockAreasProperties::SpawnChance.as_str()]
        {
            return rand::random::<f32>() <= spawn_chance;
        }

        false
    }
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_layer_tiles.iter().for_each(|layer| {
            layer.iter().for_each(|ArenaTile(x, y, tile_id)| {
                let transform = c.transform.trans(*x as f64, *y as f64);
                if let Some(TextureData { texture, src_rect }) =
                    self.textures.get_texture_data(*tile_id)
                {
                    let mut sprite = Sprite::from_texture_rect(texture, src_rect);
                    sprite.set_anchor(0.0, 0.0);
                    sprite.draw(transform, g);
                }
            });
        });
    }
}