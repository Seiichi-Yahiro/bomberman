use crate::arena::tile::Tile;
use crate::traits::FromRON;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Arenas(pub Vec<Arena>);

#[derive(Deserialize, Debug, Clone)]
pub struct Arena {
    name: String,
    width: u8,
    height: u8,
    tiles: Vec<Tile>,
}

impl FromRON for Arenas {}
