use crate::arena::tile::Tile;
use crate::traits::FromRON;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Arenas(pub Vec<Arena>);

#[derive(Deserialize, Debug, Clone)]
pub struct Arena {
    pub name: String,
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Tile>,
}

impl FromRON for Arenas {}
