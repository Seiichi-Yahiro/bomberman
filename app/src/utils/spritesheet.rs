use crate::utils::{load_tileset, load_tileset_textures, SpritesheetTextureHolder, TextureData};
use std::collections::HashMap;
use std::rc::Rc;

struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

struct SpritesheetData {
    textures: SpritesheetTextureHolder,
    texture_names_to_ids: HashMap<String, u32>,
    animations: HashMap<u32, Option<Vec<Frame>>>,
}

impl SpritesheetData {
    pub fn new(folder: &str, tileset_file: &str) -> SpritesheetData {
        let tileset = load_tileset(folder, tileset_file);
        let textures = load_tileset_textures(&tileset, folder);
        let texture_names_to_ids = Self::map_texture_names_to_ids(&tileset);
        let animations = Self::get_animations_from_tileset(&tileset);

        SpritesheetData {
            textures,
            texture_names_to_ids,
            animations,
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

    fn get_animations_from_tileset(tileset: &tiled::Tileset) -> HashMap<u32, Option<Vec<Frame>>> {
        tileset
            .tiles
            .iter()
            .map(|tile| {
                let frames = tile.animation.as_ref().map(Self::convert_tiled_frames);
                (tile.id, frames)
            })
            .collect()
    }

    fn convert_tiled_frames(frames: &Vec<tiled::Frame>) -> Vec<Frame> {
        frames
            .iter()
            .map(|frame| unsafe { std::mem::transmute(frame.clone()) })
            .collect()
    }
}

pub struct Spritesheet {
    data: Rc<SpritesheetData>,
    current_tile_id: u32,
    default_tile_id: u32,
    animation_time: f64,
    animation_length: f64,
    pub is_animating: bool,
}

impl Spritesheet {
    pub fn new(folder: &str, tileset_file: &str, default_texture: &str) -> Spritesheet {
        let data = Rc::new(SpritesheetData::new(folder, tileset_file));
        let default_tile_id = data.texture_names_to_ids[default_texture];
        let animation_length = Self::calculate_animation_length(&data.animations[&default_tile_id]);

        Spritesheet {
            data,
            default_tile_id,
            current_tile_id: default_tile_id,
            animation_time: 0.0,
            animation_length,
            is_animating: false,
        }
    }

    pub fn from_spritesheet(spritesheet: &Spritesheet) -> Spritesheet {
        Spritesheet {
            data: Rc::clone(&spritesheet.data),
            current_tile_id: spritesheet.default_tile_id,
            default_tile_id: spritesheet.default_tile_id,
            animation_time: 0.0,
            animation_length: spritesheet.animation_length,
            is_animating: false,
        }
    }

    pub fn get_current_texture_data(&self) -> Option<TextureData> {
        self.data.textures.get_texture_data(self.current_tile_id)
    }

    pub fn set_current_texture(&mut self, texture_name: &str) {
        let default_tile_id = self.data.texture_names_to_ids[texture_name];

        if self.default_tile_id != default_tile_id {
            self.default_tile_id = default_tile_id;
            self.current_tile_id = default_tile_id;
            self.animation_length =
                Self::calculate_animation_length(&self.data.animations[&default_tile_id]);
        }
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
        if let (Some(frames), true) = (
            &self.data.animations[&self.default_tile_id],
            self.is_animating,
        ) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut spritesheet = {
            let default_tile_id = 2;

            let spritesheet_data = {
                let tileset = tiled::parse_tileset(tileset_str.as_bytes(), 1).unwrap();
                SpritesheetData {
                    textures: SpritesheetTextureHolder::default(),
                    texture_names_to_ids: Default::default(),
                    animations: SpritesheetData::get_animations_from_tileset(&tileset),
                }
            };

            let animation_length = Spritesheet::calculate_animation_length(
                &spritesheet_data.animations[&default_tile_id],
            );

            Spritesheet {
                data: Rc::new(spritesheet_data),
                default_tile_id,
                current_tile_id: default_tile_id,
                animation_time: 0.0,
                animation_length,
                is_animating: false,
            }
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
