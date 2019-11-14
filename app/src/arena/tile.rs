use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Tile {
    Ground(String),
    Wall(String),
    SoftWall(String),
    SoftWallArea(String),
    PlayerStart(String),
}
