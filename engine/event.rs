use crate::tileset::{TileId, TilePosition};
use uuid::Uuid;

pub type EventId = Uuid;

pub struct Event {
    pub id: EventId,
    pub position: TilePosition,
    // pub move_speed
    // pub no_clip: bool
    pub tile_id: TileId,
    pub direction: Direction,
}

pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}
