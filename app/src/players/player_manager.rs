use crate::players::Player;
use crate::traits::game_loop_event::*;
use crate::utils::load_tileset_textures;
use graphics::math::{add, Vec2d};
use graphics::Transformed;
use piston::input::*;
use sprite::Sprite;
use std::collections::HashMap;
use std::rc::Rc;

const TEXTURE_FOLDER: &str = "app/assets/textures/player/";
const TILE_SET_NAME: &str = "player_tiles.xml";

pub struct PlayerManager {
    player: Player,
    direction_key: Option<Key>,
}

impl PlayerManager {
    pub fn new(player_spawns: HashMap<i32, Vec2d>) -> PlayerManager {
        let tileset = tiled::parse_tileset(
            std::fs::File::open(format!("{}{}", TEXTURE_FOLDER, TILE_SET_NAME)).unwrap(),
            1,
        )
        .unwrap();

        PlayerManager {
            player: Player {
                texture: load_tileset_textures(&tileset, TEXTURE_FOLDER),
                position: player_spawns[&0],
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
            let texture = Rc::clone(&self.player.texture[&2]);
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
