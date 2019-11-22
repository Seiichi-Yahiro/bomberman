use crate::utils::Spritesheet;
use graphics::math::Vec2d;

pub mod texture_names {
    pub const PLAYER_STANDING_DOWN: &str = "player_standing_down";
    pub const PLAYER_STANDING_UP: &str = "player_standing_up";
    pub const PLAYER_STANDING_LEFT: &str = "player_standing_left";
    pub const PLAYER_STANDING_RIGHT: &str = "player_standing_right";
}

pub struct Player {
    pub spritesheet: Spritesheet,
    pub position: Vec2d,
    pub speed: Vec2d,
}
