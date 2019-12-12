use crate::animation::Animatable;
use crate::asset_storage::AssetStorage;
use crate::texture_holder::SpriteTextureDataExt;
use crate::tileset::Tileset;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::Context;
use opengl_graphics::{GlGraphics, Texture};
use sprite::Sprite as PistonSprite;
use std::rc::Rc;

pub struct Sprite {
    sprite: PistonSprite<Texture>,
    tileset: Rc<Tileset>,
    current_tile_id: u32,
    default_tile_id: u32,
    animation_time: f64,
    animation_length: f64,
    is_animating: bool,
}

impl Sprite {
    pub fn from_tileset(tileset: Rc<Tileset>, tile_id: u32) -> Sprite {
        Sprite {
            sprite: PistonSprite::from_texture_data(
                tileset.texture_holder.get_texture_data(tile_id).unwrap(),
            ),
            current_tile_id: tile_id,
            default_tile_id: tile_id,
            animation_time: 0.0,
            is_animating: false,
            animation_length: Self::calculate_animation_length(&tileset, tile_id),
            tileset,
        }
    }

    fn calculate_animation_length(tileset: &Tileset, tile_id: u32) -> f64 {
        tileset
            .animation_frames_holder
            .get(&tile_id)
            .map_or(0.0, |frames| {
                frames
                    .iter()
                    .fold(0.0, |acc, item| acc + item.duration as f64)
            })
    }
}

impl Animatable for Sprite {
    fn start_animation(&mut self) {
        self.is_animating = true;
        self.animation_time = 0.0;
    }

    fn stop_animation(&mut self) {
        self.is_animating = false;
        self.current_tile_id = self.default_tile_id;
    }

    fn is_animating(&self) -> bool {
        self.is_animating
    }

    fn update_animation(&mut self, dt: f64) {
        if !self.is_animating() {
            return;
        }

        if let Some(frames) = self
            .tileset
            .animation_frames_holder
            .get(&self.default_tile_id)
        {
            self.animation_time = (self.animation_time + dt * 1000.0) % self.animation_length;

            let frame_index = frames
                .iter()
                .scan(0.0, |acc, frame| {
                    *acc += frame.duration as f64;
                    Some(*acc)
                })
                .enumerate()
                .find_map(|(index, duration)| {
                    if self.animation_time < duration {
                        Some(index)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);

            self.current_tile_id = self.frames[frame_index].tile_id;
        }
    }
}

impl Updatable for Sprite {
    fn update(&mut self, asset_storage: &mut AssetStorage, dt: f64) {
        self.update_animation(dt);
    }
}

impl Drawable for Sprite {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.sprite.draw(c.transform, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::load_animation_frames_from_tileset;

    #[test]
    fn test_animation() {
        let tileset_str = "\
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>
            <tileset version=\"1.2\" tiledversion=\"1.3.0\" name=\"player_tiles\" tilewidth=\"32\" tileheight=\"32\" tilecount=\"12\" columns=\"0\">
             <grid orientation=\"orthogonal\" width=\"1\" height=\"1\"/>
             <tile id=\"0\">
              <image width=\"32\" height=\"32\" source=\"bomber_down_left.png\"/>
             </tile>
             <tile id=\"1\">
              <image width=\"32\" height=\"32\" source=\"bomber_down_right.png\"/>
             </tile>
             <tile id=\"2\">
              <properties>
               <property name=\"name\" value=\"player_standing_down\"/>
              </properties>
              <image width=\"32\" height=\"32\" source=\"bomber_down_standing.png\"/>
              <animation>
               <frame tileid=\"0\" duration=\"200\"/>
               <frame tileid=\"2\" duration=\"200\"/>
               <frame tileid=\"1\" duration=\"200\"/>
               <frame tileid=\"2\" duration=\"200\"/>
              </animation>
             </tile>
            </tileset>
        ";

        let gid = 1;

        let tileset = tiled::parse_tileset(tileset_str.as_bytes(), gid).unwrap();
        let tileset = Tileset {
            texture_holder: Default::default(),
            animation_frames_holder: load_animation_frames_from_tileset(&tileset),
        };
        let mut sprite = Sprite::from_tileset(Rc::new(tileset), 2 + gid);

        let mut result = Vec::new();

        sprite.start_animation();
        result.push(sprite.current_tile_id);

        sprite.update_animation(100.0 / 1000.0);
        result.push(sprite.current_tile_id);

        sprite.update_animation(100.0 / 1000.0);
        result.push(sprite.current_tile_id);

        sprite.update_animation(200.0 / 1000.0);
        result.push(sprite.current_tile_id);

        sprite.update_animation(200.0 / 1000.0);
        result.push(sprite.current_tile_id);

        sprite.update_animation(200.0 / 1000.0);
        result.push(sprite.current_tile_id);

        assert_eq!(
            result.as_ref(),
            [2 + gid, 0 + gid, 2 + gid, 1 + gid, 2 + gid, 0 + gid]
        );
    }
}
