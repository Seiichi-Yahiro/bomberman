use crate::animation::Animation;
use crate::tileset::TileId;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenPosition {
    pub x: f64,
    pub y: f64,
}

impl ScreenPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layer(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurrentTileId(pub TileId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DefaultTileId(pub TileId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TilesetType<'s> {
    Tilemap,
    Tileset(&'s str),
}

#[derive(Clone, Debug)]
pub enum AnimationType {
    Shared(Option<Arc<RwLock<Animation>>>),
    Ownd(Option<Animation>),
}
