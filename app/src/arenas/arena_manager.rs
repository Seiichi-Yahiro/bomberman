use crate::arenas::object_groups;
use crate::traits::game_loop_event::*;
use graphics::Transformed;
use opengl_graphics::{Texture, TextureSettings};
use sprite::Sprite;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

const ARENAS_FOLDER: &str = "app/assets/arenas/";
const TEXTURE_FOLDER: &str = "app/assets/textures/arena_tiles/";
const FILE_NAME: &str = "arena_classic.tmx";

struct ArenaTile(pub u32, pub u32, pub u32); // x, y, tile_id
type SoftBlockAreas<'a> = HashMap<[u32; 2], &'a tiled::Object>;
type TextureMap = HashMap<u32, Rc<Texture>>;

pub struct ArenaManager {
    arena_tiles: Vec<ArenaTile>,
    textures: TextureMap,
}

impl ArenaManager {
    pub fn new() -> ArenaManager {
        let tile_map = {
            let path = format!("{}{}", ARENAS_FOLDER, FILE_NAME);
            tiled::parse_file(&Path::new(&path)).unwrap()
        };

        ArenaManager {
            arena_tiles: Self::init_arena_tiles(&tile_map),
            textures: Self::load_textures(&tile_map),
        }
    }

    fn load_textures(tile_map: &tiled::Map) -> TextureMap {
        tile_map.tilesets[0]
            .tiles
            .iter()
            .map(|tile| {
                let path = format!("{}{}", TEXTURE_FOLDER, tile.images.first().unwrap().source);
                let texture_settings = TextureSettings::new();
                let texture = Texture::from_path(path, &texture_settings).unwrap();
                (tile.id, Rc::new(texture))
            })
            .collect()
    }

    fn init_arena_tiles(tile_map: &tiled::Map) -> Vec<ArenaTile> {
        let soft_block_areas = Self::get_soft_block_areas(tile_map);
        let (tile_width, tile_height) = (tile_map.tile_width, tile_map.tile_height);

        tile_map.layers[0]
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
                        if let Some(soft_block) = soft_block_areas.get(&[x, y]) {
                            if Self::should_spawn_soft_block(soft_block) {
                                return ArenaTile(x, y, soft_block.gid - 1);
                            }
                        }

                        ArenaTile(x, y, tile - 1)
                    })
                    .collect::<Vec<ArenaTile>>()
            })
            .collect()
    }

    fn get_soft_block_areas(tile_map: &tiled::Map) -> SoftBlockAreas {
        tile_map
            .object_groups
            .iter()
            .filter(|group| group.name == object_groups::soft_block_areas::NAME)
            .flat_map(|group| &group.objects)
            .map(|object| {
                (
                    [object.x as u32, object.y as u32 - tile_map.tile_height], // subtract tile height as object tiles have their origin in the bottom left corner
                    object,
                )
            })
            .collect()
    }

    fn should_spawn_soft_block(soft_block: &tiled::Object) -> bool {
        if let tiled::PropertyValue::FloatValue(spawn_chance) =
            soft_block.properties[object_groups::soft_block_areas::properties::SPAWN_CHANCE]
        {
            return rand::random::<f32>() <= spawn_chance;
        }

        false
    }
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_tiles
            .iter()
            .for_each(|ArenaTile(x, y, tile_id)| {
                let transform = c.transform.trans(*x as f64, *y as f64);

                let texture = Rc::clone(&self.textures[tile_id]);
                let mut sprite = Sprite::from_texture(texture);
                sprite.set_anchor(0.0, 0.0);
                sprite.draw(transform, g);
            });
    }
}
