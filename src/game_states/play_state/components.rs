use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerId};
use crate::tiles::tileset::TileId;
use legion::entity::Entity;
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};
use piston::input::Button;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenPosition(pub [f64; 2]);

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
pub struct Animation(pub crate::tiles::animation::Animation);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Collision(pub bool);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerEntity(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BombEntity(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SoftBlockEntity(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HardBlockEntity(pub Entity);

#[derive(Clone, Debug)]
pub struct DeactivatedCommands(pub HashSet<PlayerCommand>);
