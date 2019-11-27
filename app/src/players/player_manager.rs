use crate::players::{MoveDirection, Player, PlayerId, PlayerTextureName};
use crate::traits::game_loop_event::*;
use crate::utils::{Spritesheet, TextureData};
use graphics::math::{add, Vec2d};
use graphics::Transformed;
use piston::input::*;
use sprite::Sprite;
use std::collections::HashMap;

const TEXTURE_FOLDER: &str = "app/assets/textures/player/";
const TILE_SET_NAME: &str = "player1.xml";

pub struct PlayerManager {
    players: Vec<Player>,
}

impl PlayerManager {
    pub fn new(player_spawns: HashMap<PlayerId, Vec2d>) -> PlayerManager {
        let spritesheet = Spritesheet::new(
            TEXTURE_FOLDER,
            TILE_SET_NAME,
            PlayerTextureName::FaceDown.as_str(),
        );

        PlayerManager {
            players: vec![
                Player::new(
                    PlayerId::Player1,
                    player_spawns[&PlayerId::Player1],
                    Spritesheet::from_spritesheet(&spritesheet),
                ),
                Player::new(
                    PlayerId::Player2,
                    player_spawns[&PlayerId::Player2],
                    spritesheet,
                ),
            ],
        }
    }

    fn update_player_speed(player: &mut Player, update_args: &GameLoopUpdateArgs) {
        let speed = 32.0 * update_args.dt;
        player.speed = match player
            .move_direction_stack
            .last()
            .unwrap_or(&MoveDirection::Standing)
        {
            MoveDirection::Up => [0.0, -speed],
            MoveDirection::Down => [0.0, speed],
            MoveDirection::Left => [-speed, 0.0],
            MoveDirection::Right => [speed, 0.0],
            MoveDirection::Standing => [0.0, 0.0],
        }
    }

    fn update_player_texture(player: &mut Player) {
        let [vx, vy] = player.speed;

        if vx > 0.0 {
            player
                .spritesheet
                .set_current_texture(PlayerTextureName::FaceRight.as_str());
        } else if vx < 0.0 {
            player
                .spritesheet
                .set_current_texture(PlayerTextureName::FaceLeft.as_str());
        } else if vy > 0.0 {
            player
                .spritesheet
                .set_current_texture(PlayerTextureName::FaceDown.as_str());
        } else if vy < 0.0 {
            player
                .spritesheet
                .set_current_texture(PlayerTextureName::FaceUp.as_str());
        };

        if vx == 0.0 && vy == 0.0 {
            player.spritesheet.stop_animation();
        } else if !player.spritesheet.is_animating {
            player.spritesheet.start_animation();
        }
    }

    fn update_player_position(player: &mut Player) {
        player.position = add(player.position, player.speed);
    }

    fn update_player_animation(player: &mut Player, dt: f64) {
        player.spritesheet.update_animation(dt);
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            self.players.iter_mut().for_each(|player| {
                let movement_direction = player.get_move_direction(&key);

                if movement_direction != MoveDirection::Standing
                    && !player.move_direction_stack.contains(&movement_direction)
                {
                    player.move_direction_stack.push(movement_direction);
                }
            });
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            self.players.iter_mut().for_each(|player| {
                let movement_direction = player.get_move_direction(&key);

                if let Some(index) = player
                    .move_direction_stack
                    .iter()
                    .position(|stored_move_direction| *stored_move_direction == movement_direction)
                {
                    player.move_direction_stack.remove(index);
                }
            });
        }
    }

    fn update(&mut self, update_args: &GameLoopUpdateArgs) {
        self.players.iter_mut().for_each(|player| {
            Self::update_player_speed(player, update_args);
            Self::update_player_texture(player);
            Self::update_player_position(player);
            Self::update_player_animation(player, update_args.dt);
        });
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.players.iter().for_each(|player| {
            let TextureData {texture, src_rect} = player.spritesheet.get_current_texture_data();
            let mut sprite = Sprite::from_texture_rect(texture, src_rect);
            
            sprite.set_anchor(0.0, 0.0);

            let transform = {
                let [x, y] = player.position;
                c.transform.trans(x, y)
            };

            sprite.draw(transform, g);
        });
    }
}
