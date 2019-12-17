use crate::animation::Animation;
use crate::texture_holder::SpriteTextureDataExt;
use crate::tile::{LayerTilesHolder, Tile};
use crate::tilemap::Tilemap;
use crate::tileset::TilePosition;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::rc::Rc;

pub struct Map {
    pub tiles: Vec<LayerTilesHolder>,
    pub tilemap: Rc<Tilemap>,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        Map {
            tiles: tilemap
                .tiles
                .iter()
                .enumerate()
                .map(|(layer_index, layer)| {
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
        }
    }

    /*pub fn update_tiles(&mut self, tile_updates: Vec<TileUpdate>) {
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
                if let Some(event) = layer.get_mut_event_by_position(tile_update.position) {
                    event.sprite_holder.sprite.update_texture_data(texture_data);
                    event.sprite_holder.animation = tileset
                        .animation_frames_holder
                        .get(&tile_update.tile_id)
                        .map(|frames| Animation::new(Rc::clone(frames)));
                } else if let Some(event) =
                    MapEvent::from_tileset(tileset, tile_update.tile_id, tile_update.layer)
                {
                    let id = event.id;
                    layer.insert(event);
                    layer.set_position(id, tile_update.position);
                }
            }
        }
    }*/
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {
        self.tiles.iter_mut().for_each(|layer| {
            layer.update(dt);
        });
    }
}

impl Drawable for Map {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|layer| {
            layer.draw(c, g);
        });
    }
}

/*pub struct TileUpdate {
    pub layer: usize,
    pub position: TilePosition,
    pub tile_id: u32,
}*/
