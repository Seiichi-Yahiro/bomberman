use crate::arena::tile::Tile;
use crate::traits::FromRON;
use rand::Rng;
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

impl Arena {
    pub fn init(&self) -> Arena {
        let mut arena = self.clone();
        let mut rng = rand::thread_rng();

        arena.tiles = arena
            .tiles
            .into_iter()
            .map(|tile| {
                if let Tile::SoftWallArea(wall_texture_id, ground_texture_id) = tile {
                    return if rng.gen_range(0, 100) <= 60 {
                        Tile::SoftWall(wall_texture_id.clone())
                    } else {
                        Tile::Ground(ground_texture_id.clone())
                    };
                }

                tile
            })
            .collect();
        arena
    }
}

impl FromRON for Arenas {}
