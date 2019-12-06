use crate::texture_holder::{SpriteTextureDataExt, TextureHolder};
use crate::traits::game_loop_event::*;
use opengl_graphics::Texture;
use sprite::Sprite;
use std::collections::HashMap;

pub struct Character {
    pub sprite: Sprite<Texture>,
    pub texture_holder: TextureHolder,
    pub tile_data: HashMap<u32, tiled::Tile>,
    face_direction_to_tile_id: HashMap<PlayerFaceDirection, u32>,
}

impl Character {
    pub fn new(path: &str, texture_folder: &str) -> Character {
        let tileset = tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap();
        let texture_holder = TextureHolder::from_tileset(&tileset, texture_folder);
        let face_direction_to_tile_id = Self::map_face_directions_to_tile_ids(&tileset);
        let tile_data = tileset
            .tiles
            .iter()
            .map(|tile| (tile.id, tile.clone()))
            .collect();
        let mut sprite = {
            let tile_id = face_direction_to_tile_id
                .get(&PlayerFaceDirection::Down)
                .unwrap();
            let texture_data = texture_holder.get_texture_data(*tile_id).unwrap();
            Sprite::from_texture_data(texture_data)
        };
        sprite.set_anchor(0.0, 0.0);
        sprite.set_position(0.0, 0.0);

        Character {
            texture_holder,
            tile_data,
            sprite,
            face_direction_to_tile_id,
        }
    }

    pub fn set_face_direction(&mut self, face_direction: PlayerFaceDirection) {
        if let Some(tile_id) = self.face_direction_to_tile_id.get(&face_direction) {
            if let Some(texture_data) = self.texture_holder.get_texture_data(*tile_id) {
                self.sprite.update_texture_data(texture_data);
            }
        }
    }

    fn map_face_directions_to_tile_ids(
        tileset: &tiled::Tileset,
    ) -> HashMap<PlayerFaceDirection, u32> {
        tileset
            .tiles
            .iter()
            .filter_map(|tile| {
                tile.properties.get("face_direction").and_then(
                    |property_value| match property_value {
                        tiled::PropertyValue::StringValue(face_direction) => Some((
                            PlayerFaceDirection::from(face_direction.as_str()),
                            tile.id + tileset.first_gid,
                        )),
                        _ => None,
                    },
                )
            })
            .collect()
    }
}

impl GameLoopEvent<()> for Character {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.sprite.draw(c.transform, g);
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum PlayerFaceDirection {
    Down,
    Up,
    Left,
    Right,
}

impl From<&str> for PlayerFaceDirection {
    fn from(face_direction: &str) -> Self {
        match face_direction {
            "down" => PlayerFaceDirection::Down,
            "up" => PlayerFaceDirection::Up,
            "left" => PlayerFaceDirection::Left,
            "right" => PlayerFaceDirection::Right,
            _ => panic!(format!(
                "Cannot create PlayerFaceDirection from {}",
                face_direction
            )),
        }
    }
}
