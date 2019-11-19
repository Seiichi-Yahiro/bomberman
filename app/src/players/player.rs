use crate::generated::player_sprite_sheet::PlayerSpriteSheet;
use graphics::math::Vec2d;

pub struct Player {
    pub spritesheet: PlayerSpriteSheet,
    pub position: Vec2d
}
