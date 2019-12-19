use crate::sprite_holder::SpriteHolder;
use crate::tile::{LayerTilesHolder, Tile};
use crate::tile_layer::TileLayer;
use crate::tilemap::Tilemap;
use crate::tileset::TileId;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Map {
    pub tiles: Vec<TileLayer>,
    pub tilemap: Rc<Tilemap>,
    sprites: HashMap<TileId, SpriteHolder>,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        Map {
            tiles: tilemap
                .tiles
                .iter()
                .enumerate()
                .map(|(layer_index, layer)| {
                    let mut tile_layer = TileLayer::new();
                    let mut map_events_holder = LayerTilesHolder::new();

                    for (&position, &tile_id) in layer.iter() {
                        if let Some(mut map_event) =
                            Tile::from_tileset(Rc::clone(&tilemap.tileset), tile_id, layer_index)
                        {
                            map_event
                                .sprite_holder
                                .sprite
                                .set_position(position[0] as f64, position[1] as f64);
                            map_events_holder.insert(map_event);
                        }
                    }

                    map_events_holder
                })
                .collect(),
            tilemap,
            sprites: tilemap.tiles.iter().flat_map(|layer| {
                layer.
            }).collect(),
        }
    }
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {
        self.tiles.iter_mut().for_each(|layer| {
            layer.update(dt);
        });
    }
}

impl Drawable for Map {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|layer| {
            layer.draw(transform, g);
        });
    }
}
