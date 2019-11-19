use crate::arenas::tile::Tile;
use crate::traits::FromRON;
use graphics::math::Vec2d;
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
                        Tile::SoftWall(wall_texture_id)
                    } else {
                        Tile::Ground(ground_texture_id)
                    };
                }

                tile
            })
            .collect();
        arena
    }

    pub fn get_player_spawns(&self) -> Vec<Vec2d> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_index, tile)| {
                if let Tile::PlayerSpawn(_) = tile {
                    return true;
                }

                false
            })
            .map(|(index, _tile)| {
                let y = (index as f64 / self.width as f64).floor();
                let x = index as f64 % self.width as f64;
                [x * 32.0, y * 32.0]
            })
            .collect()
    }
}

impl FromRON for Arenas {}
