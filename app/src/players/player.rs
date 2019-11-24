use crate::utils::Spritesheet;
use graphics::math::Vec2d;
use piston::input::Key;

pub enum TextureNames {
    StandingDown,
    StandingUp,
    StandingLeft,
    StandingRight,
}

impl TextureNames {
    pub fn as_str(&self) -> &str {
        match self {
            TextureNames::StandingDown => "player_standing_down",
            TextureNames::StandingUp => "player_standing_up",
            TextureNames::StandingLeft => "player_standing_left",
            TextureNames::StandingRight => "player_standing_right",
        }
    }
}

pub struct Player {
    pub spritesheet: Spritesheet,
    pub position: Vec2d,
    pub speed: Vec2d,
    pub movement_key_stack: Vec<Key>,
}
