use crate::generated::player_sprite_sheet::PlayerSpriteSheet;
use crate::players::Player;
use crate::traits::game_loop_event::*;
use graphics::math::Vec2d;
use graphics::Transformed;
use sprite::Sprite;
use std::rc::Rc;

pub struct PlayerManager {
    player: Player,
}

impl PlayerManager {
    pub fn new(player_spawns: Vec<Vec2d>) -> PlayerManager {
        PlayerManager {
            player: Player {
                spritesheet: PlayerSpriteSheet::new(),
                position: player_spawns[0],
            },
        }
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let mut sprite = {
            let texture = Rc::clone(&self.player.spritesheet.texture);
            let rect = *self.player.spritesheet.bomber_down_standing;
            Sprite::from_texture_rect(texture, rect)
        };

        sprite.set_anchor(0.0, 0.0);

        let transform = {
            let [x, y] = self.player.position;
            c.transform.trans(x, y)
        };

        sprite.draw(transform, g);
    }
}
