use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerId};
use crate::tiles::animation::Animation;
use crate::tiles::tileset::TileId;
use piston::input::Button;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenPosition(pub [f64; 2]);

impl ScreenPosition {
    pub fn translate(&self, p: [f64; 2]) -> Self {
        let [x, y] = self.0;
        let [px, py] = p;
        Self([x + px, y + py])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviousScreenPosition(pub [f64; 2]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XMapPosition(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct YMapPosition(pub u32);

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
    Shared(Arc<RwLock<Animation>>),
    Ownd(Animation),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player(pub PlayerId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Speed(pub f64);

// Stores width and height
// Use with ScreenPosition
#[derive(Clone, Copy, Debug)]
pub struct HitBox(pub [f64; 2]);

/*impl HitBox {
    pub fn contains(&self, screen_position: ScreenPosition) -> bool {
        let [x, y, width, height] = self.0;
        let [px, py] = screen_position.0;
        px >= x && px < x + width && py >= y && py < y + height
    }
}*/

pub struct Tilemap(pub Arc<crate::tiles::tilemap::Tilemap>);
pub struct Tileset(pub Arc<crate::tiles::tileset::Tileset>);
