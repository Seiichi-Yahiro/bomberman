use crate::sprite_holder::SpriteHolder;
use crate::tileset::{TileId, TilePosition, Tileset};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub type EventId = Uuid;

pub struct Event {
    pub id: EventId,
    pub sprite_holder: SpriteHolder,
    pub layer: usize,
    //direction: Direction,
}

impl Event {
    pub fn from_tileset(tileset: Rc<Tileset>, tile_id: u32, layer: usize) -> Event {
        Event {
            id: Uuid::new_v4(),
            sprite_holder: SpriteHolder::from_tileset(tileset, tile_id).unwrap_or_else(|| {
                panic!(format!(
                    "Could not create Event sprite from tile_id {}!",
                    tile_id
                ))
            }),
            layer, //direction: Direction::Down,
        }
    }
}

impl Updatable for Event {
    fn update(&mut self, dt: f64) {
        self.sprite_holder.update(dt);
    }
}

impl Drawable for Event {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.sprite_holder.draw(c, g);
    }
}

#[derive(Default)]
pub struct EventsHolder {
    events: HashMap<EventId, Event>,
    event_positions: HashMap<TilePosition, EventId>,
}

impl EventsHolder {
    pub fn new() -> EventsHolder {
        EventsHolder::default()
    }

    pub fn insert(&mut self, event: Event) -> Option<Event> {
        let (x, y) = event.sprite_holder.sprite.get_position();
        self.event_positions.insert([x as u32, y as u32], event.id);
        self.events.insert(event.id, event)
    }

    pub fn remove(&mut self, id: EventId) -> Option<Event> {
        self.events.remove(&id).map(|event| {
            let (x, y) = event.sprite_holder.sprite.get_position();
            self.event_positions.remove(&[x as u32, y as u32]);
            event
        })
    }

    pub fn set_position(&mut self, id: EventId, position: TilePosition) {
        self.remove(id).and_then(|event| self.insert(event));
    }

    pub fn get_event_by_id(&self, id: EventId) -> Option<&Event> {
        self.events.get(&id)
    }

    pub fn get_mut_event_by_id(&mut self, id: EventId) -> Option<&mut Event> {
        self.events.get_mut(&id)
    }

    pub fn get_event_by_position(&self, position: TilePosition) -> Option<&Event> {
        self.event_positions
            .get(&position)
            .and_then(|&id| self.get_event_by_id(id))
    }

    pub fn get_mut_event_by_position(&mut self, position: TilePosition) -> Option<&mut Event> {
        self.event_positions
            .get(&position)
            .cloned()
            .and_then(move |id| self.get_mut_event_by_id(id))
    }

    pub fn group_by_layers(&self) -> Vec<Vec<&Event>> {
        self.events
            .iter()
            .fold(Vec::<Vec<&Event>>::new(), |mut acc, (_, event)| {
                acc[event.layer].push(event); // TODO index out of bounds
                acc
            })
    }
}

pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}
