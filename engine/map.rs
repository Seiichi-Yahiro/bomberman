use crate::texture_holder::TextureHolder;
use crate::traits::game_loop_event::*;
use crate::utils::flatten_2d;
use opengl_graphics::Texture;
use sprite::Sprite;
use std::collections::HashMap;

pub struct Map {
    pub texture_holder: TextureHolder,
    pub tiles: Vec<HashMap<[u32; 2], Sprite<Texture>>>,
    pub object_groups: HashMap<String, Vec<tiled::Object>>,
}

impl Map {
    pub fn new(path: &str, texture_folder: &str) -> Map {
        let tile_map = tiled::parse_file(std::path::Path::new(path)).unwrap();
        let texture_holder = TextureHolder::from_map(&tile_map, texture_folder);
        let tiles = Self::convert_tile_map_to_tiles(&tile_map, &texture_holder);
        let object_groups = Self::extract_object_groups_from_tile_map(&tile_map);

        Map {
            tiles,
            texture_holder,
            object_groups,
        }
    }

    pub fn update_tiles(&mut self, tile_updates: Vec<TileUpdate>) {
        tile_updates
            .into_iter()
            .for_each(|tile_update| self.update_tile(tile_update));
    }

    pub fn update_tile(&mut self, tile_update: TileUpdate) {
        let TileUpdate {
            layer_id,
            position,
            tile_id,
        } = tile_update;

        if let Some(texture_data) = &self.texture_holder.get_texture_data(tile_id) {
            if let Some(layer) = self.tiles.get_mut(layer_id) {
                layer
                    .entry(position)
                    .and_modify(|sprite| {
                        sprite.set_texture(texture_data.texture.clone());
                        sprite.set_src_rect(texture_data.src_rect);
                    })
                    .or_insert_with(|| {
                        let mut sprite = Sprite::from_texture_rect(
                            texture_data.texture.clone(),
                            texture_data.src_rect,
                        );
                        let [x, y] = position;
                        sprite.set_anchor(0.0, 0.0);
                        sprite.set_position(x as f64, y as f64);
                        sprite
                    });
            }
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

    fn extract_object_groups_from_tile_map(
        tile_map: &tiled::Map,
    ) -> HashMap<String, Vec<tiled::Object>> {
        tile_map
            .object_groups
            .iter()
            .map(|group| {
                (
                    group.name.clone(),
                    group
                        .objects
                        .clone()
                        .into_iter()
                        .map(|mut object| {
                            object.y -= tile_map.tile_height as f32; // Objects origin is at bottom left
                            object
                        })
                        .collect(),
                )
            })
            .collect()
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

pub struct TileUpdate {
    layer_id: usize,
    position: [u32; 2],
    tile_id: u32,
}

impl TileUpdate {
    pub fn new(layer_id: usize, position: [u32; 2], tile_id: u32) -> TileUpdate {
        TileUpdate {
            layer_id,
            position,
            tile_id,
        }
    }
}
