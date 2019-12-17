use crate::sprite_holder::SpriteHolder;
use crate::tileset::{TilePosition, Tileset};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub type TileUuid = Uuid;

pub struct Tile {
    pub id: TileUuid,
    pub sprite_holder: SpriteHolder,
    pub layer: usize,
    //direction: Direction,
}

impl Tile {
    pub fn from_tileset(tileset: Rc<Tileset>, tile_id: u32, layer: usize) -> Option<Tile> {
        Some(Tile {
            id: Uuid::new_v4(),
            sprite_holder: SpriteHolder::from_tileset(tileset, tile_id)?,
            layer, //direction: Direction::Down,
        })
    }
}

impl Updatable for Tile {
    fn update(&mut self, dt: f64) {
        self.sprite_holder.update(dt);
    }
}

impl Drawable for Tile {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.sprite_holder.draw(c, g);
    }
}

#[derive(Default)]
pub struct LayerTilesHolder {
    events: HashMap<TileUuid, Tile>,
    event_positions: HashMap<TilePosition, TileUuid>,
}

impl LayerTilesHolder {
    pub fn new() -> LayerTilesHolder {
        LayerTilesHolder::default()
    }

    pub fn insert(&mut self, event: Tile) -> Option<Tile> {
        let (x, y) = event.sprite_holder.sprite.get_position();
        self.event_positions.insert([x as u32, y as u32], event.id);
        self.events.insert(event.id, event)
    }

    pub fn remove(&mut self, id: TileUuid) -> Option<Tile> {
        self.events.remove(&id).map(|event| {
            let (x, y) = event.sprite_holder.sprite.get_position();
            self.event_positions.remove(&[x as u32, y as u32]);
            event
        })
    }

    pub fn set_position(&mut self, id: TileUuid, position: TilePosition) {
        self.remove(id).and_then(|mut event| {
            let [x, y] = position;
            event.sprite_holder.sprite.set_position(x as f64, y as f64);
            self.insert(event)
        });
    }

    pub fn get_event_by_id(&self, id: TileUuid) -> Option<&Tile> {
        self.events.get(&id)
    }

    pub fn get_mut_event_by_id(&mut self, id: TileUuid) -> Option<&mut Tile> {
        self.events.get_mut(&id)
    }

    pub fn get_event_by_position(&self, position: TilePosition) -> Option<&Tile> {
        self.event_positions
            .get(&position)
            .and_then(|&id| self.get_event_by_id(id))
    }

    pub fn get_mut_event_by_position(&mut self, position: TilePosition) -> Option<&mut Tile> {
        self.event_positions
            .get(&position)
            .cloned()
            .and_then(move |id| self.get_mut_event_by_id(id))
    }
}

impl Updatable for LayerTilesHolder {
    fn update(&mut self, dt: f64) {
        self.events.iter_mut().for_each(|event| {
            event.1.sprite_holder.update(dt);
        });
    }
}

impl Drawable for LayerTilesHolder {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.events.iter().for_each(|event| {
            event.1.sprite_holder.draw(c, g);
        });
    }
}

pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}
