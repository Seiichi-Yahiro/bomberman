use crate::traits::game_loop_event::Updatable;
use std::collections::HashMap;
use std::rc::Rc;
use crate::tileset::TileId;

pub struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

pub struct Animation {
    is_stopped: bool,
    is_paused: bool,
    current_frame: usize,
    frame_time: f64,
    frames: Rc<Vec<Frame>>,
}

impl Animation {
    pub fn new(frames: Rc<Vec<Frame>>) -> Animation {
        Animation {
            is_stopped: true,
            is_paused: false,
            current_frame: 0,
            frame_time: 0.0,
            frames,
        }
    }

    pub fn get_current_tile_id(&self) -> TileId {
        self.frames[self.current_frame].tile_id
    }

    pub fn play(&mut self) {
        self.is_paused = false;
        self.is_stopped = false;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
        self.is_paused = false;
        self.current_frame = 0;
        self.frame_time = 0.0;
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn is_stopped(&self) -> bool {
        self.is_stopped
    }

    pub fn update(&mut self, dt: f64) {
        if self.is_paused || self.is_stopped {
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
                let frames: Vec<Frame> = tile
                    .animation
                    .as_ref()?
                    .iter()
                    .map(Self::convert_tiled_frames)
                    .map(|mut frame| {
                        frame.tile_id += tileset.first_gid;
                        frame
                    })
                    .collect();

                let tile_id = tile.id + tileset.first_gid;
                Some((tile_id, Rc::new(frames)))
            })
            .collect()
    }

    fn convert_tiled_frames(frame: &tiled::Frame) -> Frame {
        unsafe { std::mem::transmute(frame.clone()) }
    }
}

impl Updatable for Animation {
    fn update(&mut self, dt: f64) {
        self.update(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tileset::Tileset;

    #[test]
    fn test_animation_update() {
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

        let mut animation = Animation::new(Rc::clone(&tileset.animation_frames_holder[&(2 + gid)]));

        let mut result = Vec::new();

        result.push(animation.get_current_tile_id());
        animation.play();

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

        assert_eq!(result.as_ref(), [gid, gid, 2 + gid, 1 + gid, 2 + gid, gid]);
    }
}
