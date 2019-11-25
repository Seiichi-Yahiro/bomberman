use crate::utils::{load_tileset, load_tileset_textures, TextureMap};
use opengl_graphics::Texture;
use std::collections::HashMap;
use std::rc::Rc;

struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

pub struct Spritesheet {
    tileset: tiled::Tileset,
    textures: TextureMap,
    texture_names_to_ids: HashMap<String, u32>,
    current_tile_id: u32,
    default_tile_id: u32,
    animation: Option<Vec<Frame>>,
    animation_time: f64,
    animation_length: f64,
    pub is_animating: bool,
}

impl Spritesheet {
    pub fn new(folder: &str, tileset_file: &str, default_texture: &str) -> Spritesheet {
        let tileset = load_tileset(folder, tileset_file);
        let textures = load_tileset_textures(&tileset, folder);
        let texture_names_to_ids = Self::map_texture_names_to_ids(&tileset);
        let default_tile_id = texture_names_to_ids[default_texture];
        let animation = Self::get_animation(&tileset, default_tile_id);
        let animation_length = Self::calculate_animation_length(&animation);

        Spritesheet {
            default_tile_id,
            current_tile_id: default_tile_id,
            textures,
            texture_names_to_ids,
            tileset,
            animation,
            animation_time: 0.0,
            animation_length,
            is_animating: false,
        }
    }

    pub fn get_current_texture(&self) -> Rc<Texture> {
        Rc::clone(&self.textures[&self.current_tile_id])
    }

    pub fn set_current_texture(&mut self, texture_name: &str) {
        let default_tile_id = self.texture_names_to_ids[texture_name];

        if self.default_tile_id != default_tile_id {
            self.default_tile_id = default_tile_id;
            self.current_tile_id = default_tile_id;
            self.set_animation_data();
        }
    }

    fn set_animation_data(&mut self) {
        self.animation = Self::get_animation(&self.tileset, self.current_tile_id);
        self.animation_length = Self::calculate_animation_length(&self.animation);
    }

    fn get_animation(tileset: &tiled::Tileset, current_tile_id: u32) -> Option<Vec<Frame>> {
        tileset
            .tiles
            .iter()
            .find(|tile| tile.id == current_tile_id)
            .and_then(|tile| tile.animation.as_ref())
            .map(|frames| {
                frames
                    .iter()
                    .map(|frame| unsafe { std::mem::transmute(frame.clone()) }) // Transmute to my Frame as tiled::Frame fields were forgotten to be made public...
                    .collect::<Vec<Frame>>()
            })
    }

    fn calculate_animation_length(animation: &Option<Vec<Frame>>) -> f64 {
        animation.as_ref().map_or(0.0, |frames| {
            frames.iter().map(|frame| frame.duration as f64).sum()
        })
    }

    pub fn start_animation(&mut self) {
        self.is_animating = true;
        self.animation_time = 0.0;
    }

    pub fn stop_animation(&mut self) {
        self.is_animating = false;
        self.current_tile_id = self.default_tile_id;
    }

    pub fn update_animation(&mut self, dt: f64) {
        if let (Some(frames), true) = (&self.animation, self.is_animating) {
            self.animation_time = (self.animation_time + dt * 1000.0) % self.animation_length;

            let frame_index = frames
                .iter()
                .scan(0.0, |state, frame| {
                    *state += frame.duration as f64;

                    Some(*state)
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

            self.current_tile_id = frames[frame_index].tile_id;
        }
    }

    fn map_texture_names_to_ids(tileset: &tiled::Tileset) -> HashMap<String, u32> {
        tileset
            .tiles
            .iter()
            .filter_map(|tile| {
                if let Some(tiled::PropertyValue::StringValue(texture_name)) =
                    tile.properties.get("name")
                {
                    Some((texture_name.clone(), tile.id))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiled::Tileset;

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
        let tileset = tiled::parse_tileset(tileset_str.as_bytes(), 1).unwrap();
        let animation = Spritesheet::get_animation(&tileset, 2);
        let animation_length = Spritesheet::calculate_animation_length(&animation);
        let mut spritesheet = Spritesheet {
            tileset,
            textures: Default::default(),
            texture_names_to_ids: Default::default(),
            default_tile_id: 2,
            current_tile_id: 2,
            animation,
            animation_time: 0.0,
            animation_length,
            is_animating: false,
        };

        let mut result = Vec::new();

        spritesheet.start_animation();
        result.push(spritesheet.current_tile_id);

        spritesheet.update_animation(100.0 / 1000.0);
        result.push(spritesheet.current_tile_id);

        spritesheet.update_animation(100.0 / 1000.0);
        result.push(spritesheet.current_tile_id);

        spritesheet.update_animation(200.0 / 1000.0);
        result.push(spritesheet.current_tile_id);

        spritesheet.update_animation(200.0 / 1000.0);
        result.push(spritesheet.current_tile_id);

        spritesheet.update_animation(200.0 / 1000.0);
        result.push(spritesheet.current_tile_id);

        assert_eq!(result.as_ref(), [2, 0, 2, 1, 2, 0]);
    }
}
