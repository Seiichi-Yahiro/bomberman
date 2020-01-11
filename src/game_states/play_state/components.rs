use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerId};
use crate::tiles::animation::Animation;
use crate::tiles::tileset::TileId;
use legion::entity::Entity;
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};
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

    pub fn absolute_hit_box(&self, hit_box: HitBox) -> crate::tiles::tileset::HitBox {
        let [pos_x, pos_y] = self.0;
        let [x, y, w, h] = hit_box.0;
        [pos_x + x, pos_y + y, w, h]
    }

    pub fn map_position(&self, tilemap: Tilemap) -> [u32; 2] {
        let [x, y] = self.0;
        [
            x as u32 / tilemap.0.tile_width,
            y as u32 / tilemap.0.tile_height,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviousScreenPosition(pub [f64; 2]);

// Use with ScreenPosition
#[derive(Clone, Copy, Debug)]
pub struct HitBox(pub crate::tiles::tileset::HitBox);

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
pub struct MovementSpeed(pub f64);

#[derive(Clone)]
pub struct Tilemap(pub Arc<crate::tiles::tilemap::Tilemap>);

#[derive(Clone)]
pub struct Tileset(pub Arc<crate::tiles::tileset::Tileset>);

#[derive(Clone)]
pub struct AssetStorage(pub Arc<RwLock<crate::utils::asset_storage::AssetStorage>>);

#[derive(Clone, Copy, Debug)]
pub struct SpawnBomb(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodyHandle(pub DefaultBodyHandle);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColliderHandle(pub DefaultColliderHandle);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Player,
    Bomb,
    SoftBlock,
    HardBlock,
}
