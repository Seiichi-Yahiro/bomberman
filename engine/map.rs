use crate::texture_holder::{TextureData, TextureHolder};
use crate::traits::game_loop_event::*;
use crate::utils::flatten_2d;
use graphics::Transformed;
use opengl_graphics::Texture;
use sprite::Sprite;
use std::collections::HashMap;

pub struct Map {
    texture_holder: TextureHolder,
    tiles: Vec<HashMap<[u32; 2], Sprite<Texture>>>,
}

impl Map {
    pub fn new(path: &str, texture_folder: &str) -> Map {
        let tile_map = tiled::parse_file(std::path::Path::new(path)).unwrap();
        let texture_holder = TextureHolder::from_map(&tile_map, texture_folder);
        let tiles = Self::convert_tile_map_to_tiles(&tile_map, &texture_holder);

        Map {
            tiles,
            texture_holder,
        }
    }

    fn convert_tile_map_to_tiles(
        tile_map: &tiled::Map,
        texture_holder: &TextureHolder,
    ) -> Vec<HashMap<[u32; 2], Sprite<Texture>>> {
        let convert_layer_to_tiles = |layer: &tiled::Layer| {
            flatten_2d(&layer.tiles)
                .into_iter()
                .filter_map(|(row, column, &tile_id)| {
                    texture_holder
                        .get_texture_data(tile_id)
                        .map(|texture_data| {
                            let x = column as u32 * tile_map.tile_width;
                            let y = row as u32 * tile_map.tile_height;
                            let mut sprite = Sprite::from_texture_rect(
                                texture_data.texture,
                                texture_data.src_rect,
                            );
                            sprite.set_anchor(0.0, 0.0);
                            sprite.set_position(x as f64, y as f64);
                            ([x, y], sprite)
                        })
                })
                .collect()
        };

        tile_map.layers.iter().map(convert_layer_to_tiles).collect()
    }
}

impl GameLoopEvent<()> for Map {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        for layer in &self.tiles {
            for (_, sprite) in layer {
                sprite.draw(c.transform, g);
            }
        }
    }
}
