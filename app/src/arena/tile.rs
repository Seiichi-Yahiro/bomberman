use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Tile {
    Ground(String),
    Wall(String),
    SoftWall(String),
    SoftWallArea(String, String),
    PlayerStart(String),
}

impl Tile {
    pub fn get_value(&self) -> &str {
        match self {
            Tile::Ground(texture_id) => texture_id,
            Tile::Wall(texture_id) => texture_id,
            Tile::SoftWall(texture_id) => texture_id,
            Tile::SoftWallArea(wall_texture_id, ground_texture_id) => ground_texture_id,
            Tile::PlayerStart(texture_id) => texture_id,
        }
    }
}
