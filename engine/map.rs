use crate::asset_storage::{Asset, AssetStorage};
use crate::event::{Event, EventId};
use crate::sprite_holder::SpriteHolder;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, TilePosition, Tileset, TilesetId};
use crate::traits::game_loop_event::{Drawable, Updatable};
use crate::utils::flatten_2d;
use crate::world::World;
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
                        .map(|(&position, &tile_id)| {
                            (
                                position,
                                SpriteHolder::from_tileset(Rc::clone(&tilemap.tileset), tile_id)
                                    .unwrap(),
                            )
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
    fn update(&mut self, world: &mut World, dt: f64) {
        self.tiles.iter_mut().for_each(|layer| {
            layer.iter_mut().for_each(|(_, sprite)| {
                sprite.update(world, dt);
            });
        });
    }
}

impl Drawable for Map {
    fn draw(&self, world: &World, c: &Context, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|layer| {
            layer.iter().for_each(|(_, sprite)| {
                sprite.draw(world, c, g);
            });
        });
    }
}
