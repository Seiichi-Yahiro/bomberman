use crate::sprite_holder::SpriteHolder;
use crate::texture_holder::{SpriteTextureDataExt, TextureHolder};
use crate::tilemap::Tilemap;
use crate::tileset::Tileset;
use crate::traits::game_loop_event::*;
use crate::utils::flatten_2d;
use opengl_graphics::Texture;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Map {
    pub tilemap: Rc<Tilemap>,
    pub tiles: Vec<HashMap<[u32; 2], SpriteHolder>>,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        let tiles = Self::convert_tilemap_to_tiles(&tilemap);

        Map { tiles, tilemap }
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

        let tileset = &Rc::clone(&self.tilemap.tileset);

        if let Some(layer) = self.tiles.get_mut(layer_id) {
            layer
                .entry(position)
                .and_modify(|sprite_holder| {
                    if let Some(texture_data) = tileset.texture_holder.get_texture_data(tile_id) {
                        sprite_holder.sprite.update_texture_data(texture_data);
                    }
                })
                .or_insert_with(|| {
                    SpriteHolder::from_tileset(Rc::clone(tileset), tile_id).map(
                        |mut sprite_holder| {
                            let [x, y] = position;
                            sprite_holder.sprite.set_anchor(0.0, 0.0);
                            sprite_holder.sprite.set_position(x as f64, y as f64);
                            sprite_holder
                        },
                    )
                });
        }
    }

    fn convert_tilemap_to_tiles(tilemap: &Tilemap) -> Vec<HashMap<[u32; 2], SpriteHolder>> {
        tilemap
            .tiles
            .iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .filter_map(|tile| {
                        SpriteHolder::from_tileset(Rc::clone(&tilemap.tileset), tile.tile_id).map(
                            |mut sprite_holder| {
                                sprite_holder.sprite.set_anchor(0.0, 0.0);
                                sprite_holder
                                    .sprite
                                    .set_position(tile.x as f64, tile.y as f64);
                                ([tile.x, tile.y], sprite_holder)
                            },
                        )
                    })
                    .collect()
            })
            .collect()
    }
}

impl Drawable for Map {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        for layer in &self.tiles {
            for (_, sprite) in layer {
                sprite.draw(c, g);
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
