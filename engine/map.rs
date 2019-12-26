use crate::components::{CurrentTileId, DefaultTileId, Layer, MapPosition, ScreenPosition};
use crate::scene::{SceneNode, SceneNodeLink};
use crate::sprite_holder::SpriteHolder;
use crate::texture_holder::SpriteTextureDataExt;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, TilePosition};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::{Matrix2d, Scalar};
use graphics::Transformed;
use itertools::{EitherOrBoth, Itertools};
use legion::prelude::*;
use opengl_graphics::{GlGraphics, Texture};
use sprite::Sprite;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

type SpriteHolderRc = Rc<RefCell<SpriteHolder>>;
type SpriteHolders = HashMap<TileId, SpriteHolderRc>;
type TileLayer = HashMap<TilePosition, SpriteHolderRc>;
type EntityLayer = HashMap<TilePosition, SceneNodeLink>;

pub struct Map {
    pub tilemap: Rc<Tilemap>,
    pub world: RefCell<World>,
}

impl Map {
    pub fn new(tilemap: Rc<Tilemap>, world: World) -> Map {
        Map {
            tilemap,
            world: RefCell::new(world),
        }
    }

    pub fn create_tilemap_entities(&self) {
        self.tilemap
            .tiles
            .iter()
            .enumerate()
            .for_each(|(layer_index, layer)| {
                let components = layer
                    .iter()
                    .map(|(&[x, y], &tile_id)| {
                        (
                            MapPosition::new(x, y),
                            ScreenPosition::new(x as f64, y as f64),
                            DefaultTileId(tile_id),
                            CurrentTileId(tile_id),
                        )
                    })
                    .collect_vec();

                self.world
                    .borrow_mut()
                    .insert((Layer(layer_index),), components);
            });
    }
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {}
}

impl Drawable for Map {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        let mut sprite: Option<Sprite<Texture>> = None;

        for layer in 0..self.tilemap.tiles.len() {
            let layer = Layer(layer);
            let query =
                <(Read<ScreenPosition>, Read<CurrentTileId>)>::query().filter(tag_value(&layer));

            for (pos, tile_id) in query.iter(&mut self.world.borrow_mut()) {
                let texture_data = self
                    .tilemap
                    .tileset
                    .texture_holder
                    .get_texture_data(tile_id.0);

                if let Some(texture_data) = texture_data {
                    if let Some(sprite) = &mut sprite {
                        sprite.update_texture_data(texture_data);
                    } else {
                        sprite = Some(Sprite::from_texture_data(texture_data.clone()));
                    }

                    sprite
                        .as_ref()
                        .unwrap()
                        .draw(transform.trans(pos.x, pos.y), g)
                }
            }
        }
    }
}
