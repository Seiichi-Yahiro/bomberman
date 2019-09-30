use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Tile {
    Ground(u8),
    Wall(u8),
    SoftWall(u8),
    SoftWallArea(u8),
    PlayerStart(u8),
}
