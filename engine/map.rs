use crate::sprite::Sprite;
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
    pub tiles: Vec<HashMap<[u32; 2], Sprite>>,
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

        if let Some(texture_data) = self.texture_holder.get_texture_data(tile_id) {
            if let Some(layer) = self.tiles.get_mut(layer_id) {
                layer
                    .entry(position)
                    .and_modify(|sprite| {
                        sprite.update_texture_data(texture_data.clone());
                    })
                    .or_insert_with(|| {
                        let mut sprite = Sprite::from_texture_data(texture_data.clone());
                        let [x, y] = position;
                        sprite.set_anchor(0.0, 0.0);
                        sprite.set_position(x as f64, y as f64);
                        sprite
                    });
            }
        }
    }

    fn convert_tilemap_to_tiles(tilemap: &Tilemap) -> Vec<HashMap<[u32; 2], Sprite>> {
        tilemap
            .tiles
            .iter()
            .map(|tiles| {
                tiles.iter().map(|tile| {
                    let sprite = Sprite::from_tileset(Rc::clone(&tilemap.tileset), tile.tile_id);
                    let x = tile.column as u32 * tilemap.tile_width;
                    let y = tile.row as u32 * tilemap.tile_height;
                    sprite.set_anchor(0.0, 0.0);
                    sprite.set_position(x as f64, y as f64);
                    ([x, y], sprite)
                })
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
