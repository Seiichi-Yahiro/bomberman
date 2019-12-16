use crate::animation::Animation;
use crate::event::{Event, EventId};
use crate::sprite_holder::SpriteHolder;
use crate::texture_holder::SpriteTextureDataExt;
use crate::tilemap::Tilemap;
use crate::tileset::TilePosition;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Map {
    pub events: HashMap<EventId, Event>,
    pub tiles: Vec<HashMap<TilePosition, SpriteHolder>>,
    pub tilemap: Rc<Tilemap>,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        Map {
            events: HashMap::new(),
            tiles: tilemap
                .tiles
                .iter()
                .map(|layer| {
                    layer
                        .iter()
                        .filter_map(|(&position, &tile_id)| {
                            let mut sprite_holder =
                                SpriteHolder::from_tileset(Rc::clone(&tilemap.tileset), tile_id)?;
                            sprite_holder.sprite.set_anchor(0.0, 0.0);
                            let [x, y] = position;
                            sprite_holder.sprite.set_position(x as f64, y as f64);

                            Some((position, sprite_holder))
                        })
                        .collect()
                })
                .collect(),
            tilemap,
        }
    }

    pub fn update_tiles(&mut self, tile_updates: Vec<TileUpdate>) {
        tile_updates
            .into_iter()
            .for_each(|tile_update| self.update_tile(tile_update));
    }

    pub fn update_tile(&mut self, tile_update: TileUpdate) {
        if let Some(texture_data) = self
            .tilemap
            .tileset
            .texture_holder
            .get_texture_data(tile_update.tile_id)
        {
            let tileset = Rc::clone(&self.tilemap.tileset);

            if let Some(layer) = self.tiles.get_mut(tile_update.layer) {
                layer
                    .entry(tile_update.position)
                    .and_modify(|sprite_holder| {
                        sprite_holder.sprite.update_texture_data(texture_data);
                        sprite_holder.animation = tileset
                            .animation_frames_holder
                            .get(&tile_update.tile_id)
                            .map(|frames| Animation::new(Rc::clone(frames)));
                    })
                    .or_insert_with(|| {
                        let mut sprite_holder =
                            SpriteHolder::from_tileset(tileset, tile_update.tile_id)
                                .unwrap_or_else(|| {
                                    panic!(format!(
                                        "Tried to insert tile {} but could not create sprite!",
                                        tile_update.tile_id
                                    ));
                                });
                        sprite_holder.sprite.set_anchor(0.0, 0.0);
                        let [x, y] = tile_update.position;
                        sprite_holder.sprite.set_position(x as f64, y as f64);
                        sprite_holder
                    });
            }
        }
    }
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {
        self.tiles.iter_mut().for_each(|layer| {
            layer.iter_mut().for_each(|(_, sprite)| {
                sprite.update(dt);
            });
        });
    }
}

impl Drawable for Map {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|layer| {
            layer.iter().for_each(|(_, sprite)| {
                sprite.draw(c, g);
            });
        });
    }
}

pub struct TileUpdate {
    pub layer: usize,
    pub position: TilePosition,
    pub tile_id: u32,
}
