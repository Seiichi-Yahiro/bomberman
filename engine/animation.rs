use crate::asset_storage::AssetStorage;
use crate::traits::game_loop_event::{Drawable, Updatable};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

pub struct Animation {
    is_playing: bool,
    current_frame: usize,
    frame_time: f64,
    frames: Rc<Vec<Frame>>,
}

impl Animation {
    pub fn new(frames: Rc<Vec<Frame>>) -> Animation {
        Animation {
            is_playing: false,
            current_frame: 0,
            frame_time: 0.0,
            frames,
        }
    }

    pub fn get_current_tile_id(&self) -> u32 {
        self.frames[self.current_frame].tile_id
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
        self.frame_time = 0.0;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn update(&mut self, dt: f64) {
        if !self.is_playing {
            return;
        }

        self.frame_time += dt * 1000.0;
        let frame_duration = self.frames[self.current_frame].duration as f64;

        if self.frame_time >= frame_duration {
            self.frame_time -= frame_duration;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn load_animation_frames_from_tileset(
        tileset: &tiled::Tileset,
    ) -> HashMap<u32, Rc<Vec<Frame>>> {
        tileset
            .tiles
            .iter()
            .filter_map(|tile| {
                tile.animation
                    .as_ref()
                    .map(Self::convert_tiled_frames)
                    .map(|frames| {
                        frames
                            .into_iter()
                            .map(|mut frame| {
                                frame.tile_id += tileset.first_gid;
                                frame
                            })
                            .collect::<Vec<Frame>>()
                    })
                    .map(|frames| {
                        let tile_id = tile.id + tileset.first_gid;
                        (tile_id, Rc::new(frames))
                    })
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

impl Updatable for Animation {
    fn update(&mut self, _asset_storage: &mut AssetStorage, dt: f64) {
        self.update(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tileset::Tileset;

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
            animation_frames_holder: Animation::load_animation_frames_from_tileset(&tileset),
        };

        let mut animation = Animation::new(tileset.animation_frames_holder[2 + gid]);

        let mut result = Vec::new();

        animation.start();
        result.push(animation.get_current_tile_id());

        animation.update(100.0 / 1000.0);
        result.push(animation.get_current_tile_id());

        animation.update(100.0 / 1000.0);
        result.push(animation.get_current_tile_id());

        animation.update(200.0 / 1000.0);
        result.push(animation.get_current_tile_id());

        animation.update(200.0 / 1000.0);
        result.push(animation.get_current_tile_id());

        animation.update(200.0 / 1000.0);
        result.push(animation.get_current_tile_id());

        assert_eq!(
            result.as_ref(),
            [2 + gid, 0 + gid, 2 + gid, 1 + gid, 2 + gid, 0 + gid]
        );
    }
}
