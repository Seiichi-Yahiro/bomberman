use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Tile {
    Ground(String),
    Wall(String),
    SoftWall(String),
    SoftWallArea(String),
    PlayerStart(String),
}

impl Tile {
    pub fn get_value(&self) -> &str {
        match self {
            Tile::Ground(value) => value,
            Tile::Wall(value) => value,
            Tile::SoftWall(value) => value,
            Tile::SoftWallArea(value) => value,
            Tile::PlayerStart(value) => value,
        }
    }
}
