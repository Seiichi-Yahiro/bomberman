use crate::players::{Player, TextureNames};
use crate::traits::game_loop_event::*;
use crate::utils::Spritesheet;
use graphics::math::{add, Vec2d};
use graphics::Transformed;
use piston::input::*;
use sprite::Sprite;
use std::collections::HashMap;

const TEXTURE_FOLDER: &str = "app/assets/textures/player/";
const TILE_SET_NAME: &str = "player_tiles.xml";

pub struct PlayerManager {
    player: Player,
}

impl PlayerManager {
    pub fn new(player_spawns: HashMap<i32, Vec2d>) -> PlayerManager {
        PlayerManager {
            player: Player {
                spritesheet: Spritesheet::new(
                    TEXTURE_FOLDER,
                    TILE_SET_NAME,
                    TextureNames::StandingDown.as_str(),
                ),
                position: player_spawns[&0],
                speed: [0.0; 2],
                movement_key_stack: Vec::new(),
            },
        }
    }

    fn update_player_speed(&mut self, update_args: &GameLoopUpdateArgs) {
        let speed = 32.0 * update_args.dt;

        if let Some(key) = self.player.movement_key_stack.last() {
            self.player.speed = match key {
                Key::Left => [-speed, 0.0],
                Key::Right => [speed, 0.0],
                Key::Up => [0.0, -speed],
                Key::Down => [0.0, speed],
                _ => [0.0, 0.0],
            }
        } else {
            self.player.speed = [0.0, 0.0];
        }
    }

    fn update_player_texture(&mut self) {
        let [vx, vy] = self.player.speed;

        if vx > 0.0 {
            self.player
                .spritesheet
                .set_current_texture(TextureNames::StandingRight.as_str());
        } else if vx < 0.0 {
            self.player
                .spritesheet
                .set_current_texture(TextureNames::StandingLeft.as_str());
        } else if vy > 0.0 {
            self.player
                .spritesheet
                .set_current_texture(TextureNames::StandingDown.as_str());
        } else if vy < 0.0 {
            self.player
                .spritesheet
                .set_current_texture(TextureNames::StandingUp.as_str());
        };

        if vx == 0.0 && vy == 0.0 {
            self.player.spritesheet.stop_animation();
        } else if !self.player.spritesheet.is_animating {
            self.player.spritesheet.start_animation();
        }
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up | Key::Down | Key::Right | Key::Left => {
                    if self
                        .player
                        .movement_key_stack
                        .iter()
                        .position(|stored_key| *stored_key == key)
                        .is_none()
                    {
                        self.player.movement_key_stack.push(key)
                    }
                }
                _ => {}
            }
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Up | Key::Down | Key::Right | Key::Left => {
                    if let Some(index) = self
                        .player
                        .movement_key_stack
                        .iter()
                        .position(|stored_key| *stored_key == key)
                    {
                        self.player.movement_key_stack.remove(index);
                    }
                }
                _ => {}
            }
        }
    }

    fn update(&mut self, update_args: &GameLoopUpdateArgs) {
        self.update_player_speed(update_args);
        self.update_player_texture();
        self.player.position = add(self.player.position, self.player.speed);
        self.player.spritesheet.update_animation(update_args.dt);
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let mut sprite = {
            let texture = self.player.spritesheet.get_current_texture();
            Sprite::from_texture(texture)
        };

        sprite.set_anchor(0.0, 0.0);

        let transform = {
            let [x, y] = self.player.position;
            c.transform.trans(x, y)
        };

        sprite.draw(transform, g);
    }
}
