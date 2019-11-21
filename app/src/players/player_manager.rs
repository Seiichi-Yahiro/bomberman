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
    key_manager: HashMap<Key, bool>,
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
            key_manager: HashMap::new(),
        }
    }
}

impl GameLoopEvent<()> for PlayerManager {
    fn event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            self.key_manager.insert(key, true);
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            self.key_manager.insert(key, false);
        }
    }

    fn update(&mut self, dt: f64) {
        let speed = 32.0 * dt;
        let fallback_is_key_pressed = false;
        let mut velocity = [0.0, 0.0];

        [
            (Key::Right, 0, speed),
            (Key::Left, 0, -speed),
            (Key::Up, 1, -speed),
            (Key::Down, 1, speed),
        ]
        .iter()
        .for_each(|(key, direction, speed)| {
            if *self
                .key_manager
                .get(key)
                .unwrap_or(&fallback_is_key_pressed)
            {
                velocity[*direction] += *speed;
            }
        });

        self.player.position = add(self.player.position, velocity);
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
