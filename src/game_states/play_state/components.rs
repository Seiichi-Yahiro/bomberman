use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerId};
use crate::tiles::animation::Animation;
use crate::tiles::tileset::TileId;
use piston::input::Button;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
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

#[derive(Clone, Debug, PartialEq)]
pub struct MoveDirectionStack(pub Vec<Direction>);

#[derive(Clone, Debug, PartialEq)]
pub struct Controls(pub HashMap<Button, PlayerCommand>);

#[derive(Clone, Debug)]
pub enum AnimationType {
    Shared(Option<Arc<RwLock<Animation>>>),
    Ownd(Option<Animation>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player(pub PlayerId);

pub struct Tileset(pub Arc<crate::tiles::tileset::Tileset>);

#[derive(Clone, Debug, PartialEq)]
pub struct TurnCommand(pub Direction);
