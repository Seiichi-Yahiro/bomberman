use crate::generated::player_sprite_sheet::PlayerSpriteSheet;
use crate::players::Player;
use crate::traits::game_loop_event::*;
use graphics::math::{add, Vec2d};
use graphics::Transformed;
use piston::input::*;
use sprite::Sprite;
use std::rc::Rc;

pub struct PlayerManager {
    player: Player,
    direction_key: Option<Key>,
}

impl PlayerManager {
    pub fn new(player_spawns: Vec<Vec2d>) -> PlayerManager {
        PlayerManager {
            player: Player {
                spritesheet: PlayerSpriteSheet::new(),
                position: player_spawns[0],
            },
            direction_key: None,
        }
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Right | Key::Left | Key::Up | Key::Down => {
                    self.direction_key = Some(key);
                }
                _ => {}
            }
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Right | Key::Left | Key::Up | Key::Down => {
                    self.direction_key = None;
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, dt: f64) {
        if let Some(key) = self.direction_key {
            let speed = 32.0;

            self.player.position = match key {
                Key::Right => add(self.player.position, [speed * dt, 0.0]),
                Key::Left => add(self.player.position, [-speed * dt, 0.0]),
                Key::Up => add(self.player.position, [0.0, -speed * dt]),
                Key::Down => add(self.player.position, [0.0, speed * dt]),
                _ => self.player.position,
            }
        }
    }

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
