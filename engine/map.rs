use crate::scene::SceneTree;
use crate::sprite_holder::SpriteHolder;
use crate::tile::Tile;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, TilePosition};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use graphics::Transformed;
use itertools::{EitherOrBoth, Itertools};
use opengl_graphics::GlGraphics;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::rc::Rc;

type SpriteHolderRc = Rc<RefCell<SpriteHolder>>;
type SpriteHolders = HashMap<TileId, SpriteHolderRc>;
type TileLayer = HashMap<TilePosition, SpriteHolderRc>;
type EntityLayer = HashMap<TilePosition, SceneTree>;

pub struct Map {
    pub tilemap: Rc<Tilemap>,
    pub entities: Vec<EntityLayer>,
    tiles: Vec<TileLayer>,
    sprites: SpriteHolders,
}

impl Map {
    pub fn from_tilemap(tilemap: Rc<Tilemap>) -> Map {
        let sprites = Self::create_sprite_holders(&tilemap);
        let tiles = Self::create_tiles(&tilemap, &sprites);
        let entities = vec![HashMap::new(); tiles.len()];

        Map {
            tiles,
            entities,
            sprites,
            tilemap,
        }
    }

    pub fn get_tiles(&self) -> &Vec<TileLayer> {
        &self.tiles
    }

    pub fn add_entity(&mut self, layer: usize, tile: Tile) {
        let (x, y) = tile.sprite_holder.sprite.get_position();
        let tile = Rc::new(RefCell::new(tile));
        let scene_tree = SceneTree::new(tile);
        self.entities[layer].insert([x as u32, y as u32], scene_tree);
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

    fn draw_tile_layer(tile_layer: &TileLayer, transform: Matrix2d, g: &mut GlGraphics) {
        tile_layer.iter().for_each(|([x, y], sprite_holder)| {
            sprite_holder
                .borrow()
                .draw(transform.trans(*x as f64, *y as f64), g);
        });
    }

    fn draw_entity_layer(entity_layer: &EntityLayer, transform: Matrix2d, g: &mut GlGraphics) {
        entity_layer.iter().for_each(|(_, scene_tree)| {
            scene_tree.draw(transform, g);
        });
    }
}

impl Updatable for Map {
    fn update(&mut self, dt: f64) {
        self.sprites.values().for_each(|sprite_holder| {
            sprite_holder.borrow_mut().update(dt);
        });

        self.entities.iter_mut().for_each(|entity_layer| {
            entity_layer.values_mut().for_each(|scene_tree| {
                scene_tree.update(dt);
            });
        });
    }
}

impl Drawable for Map {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.tiles
            .iter()
            .zip_longest(&self.entities)
            .for_each(|maps| match maps {
                EitherOrBoth::Both(tile_layer, entity_layer) => {
                    Self::draw_tile_layer(tile_layer, transform, g);
                    Self::draw_entity_layer(entity_layer, transform, g);
                }
                EitherOrBoth::Left(tile_layer) => {
                    Self::draw_tile_layer(tile_layer, transform, g);
                }
                EitherOrBoth::Right(entity_layer) => {
                    Self::draw_entity_layer(entity_layer, transform, g);
                }
            });
    }
}
