use crate::asset_storage::{Asset, AssetStorage};
use crate::event::{Event, EventId};
use crate::sprite_holder::SpriteHolder;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, TilePosition, Tileset, TilesetId};
use crate::traits::game_loop_event::{Drawable, Updatable};
use crate::utils::flatten_2d;
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;

pub struct Map {
    pub events: HashMap<EventId, Event>,
    pub tiles: Vec<HashMap<TilePosition, SpriteHolder>>,
    tilemap: Rc<Tilemap>,
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

    pub fn get_width(&self) -> u32 {
        self.tilemap.width
    }

    pub fn get_height(&self) -> u32 {
        self.tilemap.height
    }

    pub fn get_object_groups(&self) -> &HashMap<String, Vec<tiled::Object>> {
        &self.tilemap.object_groups
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
