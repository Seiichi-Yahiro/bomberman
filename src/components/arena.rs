use crate::enums::Tile;
use amethyst::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Arena {
    pub tiles: Vec<Vec<Tile>>,
}

impl Component for Arena {
    type Storage = DenseVecStorage<Self>;
}
