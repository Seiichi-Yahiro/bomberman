use crate::players::{texture_names, Player};
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
                    texture_names::PLAYER_STANDING_DOWN,
                ),
                position: player_spawns[&0],
                speed: [0.0; 2],
            },
        }
    }

    fn update_player_speed(&mut self, update_args: &GameLoopUpdateArgs) {
        let speed = 32.0 * update_args.dt;
        let mut vx = 0.0;
        let mut vy = 0.0;

        if update_args.key_state.get(&Key::Right) {
            vx += speed;
        }

        if update_args.key_state.get(&Key::Left) {
            vx += -speed;
        }

        if update_args.key_state.get(&Key::Down) {
            vy += speed;
        }

        if update_args.key_state.get(&Key::Up) {
            vy += -speed;
        }

        let new_speed = [vx, vy];
        self.player.speed = new_speed;
    }

    fn update_player_texture(&mut self) {
        let [vx, vy] = self.player.speed;

        if vx.abs() > vy.abs() {
            if vx > 0.0 {
                self.player
                    .spritesheet
                    .set_current_texture(texture_names::PLAYER_STANDING_RIGHT);
            } else if vx < 0.0 {
                self.player
                    .spritesheet
                    .set_current_texture(texture_names::PLAYER_STANDING_LEFT);
            }
        } else if vy.abs() >= vx.abs() {
            if vy > 0.0 {
                self.player
                    .spritesheet
                    .set_current_texture(texture_names::PLAYER_STANDING_DOWN);
            } else if vy < 0.0 {
                self.player
                    .spritesheet
                    .set_current_texture(texture_names::PLAYER_STANDING_UP);
            }
        }
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn event(&mut self, _event: &Event) {}

    fn update(&mut self, update_args: &GameLoopUpdateArgs) {
        self.update_player_speed(update_args);
        self.update_player_texture();
        self.player.position = add(self.player.position, self.player.speed);
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
