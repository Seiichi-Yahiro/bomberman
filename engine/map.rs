use crate::sprite_holder::SpriteHolder;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, TilePosition};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type SpriteHolderRc = Rc<RefCell<SpriteHolder>>;
type SpriteHolders = HashMap<TileId, SpriteHolderRc>;
type TileLayer = HashMap<TilePosition, SpriteHolderRc>;

pub struct Map {
    pub tilemap: Rc<Tilemap>,
    tiles: Vec<TileLayer>,
    sprites: SpriteHolders,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        let sprites = Self::create_sprite_holders(&tilemap);

        Map {
            tiles: Self::create_tiles(&tilemap, &sprites),
            sprites,
            tilemap,
        }
    }

    pub fn get_tiles(&self) -> &Vec<TileLayer> {
        &self.tiles
    }

    fn create_tiles(tilemap: &Tilemap, sprites: &SpriteHolders) -> Vec<TileLayer> {
        tilemap
            .tiles
            .iter()
            .enumerate()
            .map(|(layer_index, layer)| {
                layer
                    .iter()
                    .filter_map(|(&position, tile_id)| {
                        let sprite_holder = sprites.get(tile_id)?;
                        let entry = (position, Rc::clone(sprite_holder));
                        Some(entry)
                    })
                    .collect()
            })
            .collect()
    }

    fn create_sprite_holders(tilemap: &Tilemap) -> SpriteHolders {
        tilemap
            .get_used_tile_ids()
            .iter()
            .filter_map(|&tile_id| {
                let tileset = Rc::clone(&tilemap.tileset);
                let sprite_holder = SpriteHolder::from_tileset(tileset, tile_id)?;
                let entry = (tile_id, Rc::new(RefCell::new(sprite_holder)));
                Some(entry)
            })
            .collect()
    }
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {
        self.sprites.values().for_each(|sprite_holder| {
            sprite_holder.borrow_mut().update(dt);
        });
    }
}

impl Drawable for Map {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|layer| {
            layer.iter().for_each(|([x, y], sprite_holder)| {
                sprite_holder
                    .borrow()
                    .draw(transform.trans(*x as f64, *y as f64), g);
            });
        });
    }
}
