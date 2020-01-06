use crate::tiles::tileset::TileId;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

#[derive(Clone, Debug)]
pub struct Animation {
    is_paused: bool,
    is_finished: bool,
    should_loop: bool,
    current_frame: usize,
    frame_time: f64,
    frames: Arc<Vec<Frame>>,
}

impl Animation {
    pub fn builder(frames: Arc<Vec<Frame>>) -> AnimationBuilder {
        AnimationBuilder::new(frames)
    }

    pub fn get_current_tile_id(&self) -> TileId {
        self.frames[self.current_frame].tile_id
    }

    pub fn play(&mut self) {
        self.is_paused = false;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn is_playing(&self) -> bool {
        !self.is_paused && !self.is_finished
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn update(&mut self, dt: f64) {
        if self.is_paused || self.is_finished {
            return;
        }

        self.frame_time += dt * 1000.0;
        let frame_duration = self.frames[self.current_frame].duration as f64;

        if self.frame_time >= frame_duration {
            self.frame_time -= frame_duration;

            self.current_frame += 1;

            if self.current_frame == self.frames.len() {
                if self.should_loop {
                    self.current_frame %= self.frames.len();
                } else {
                    self.is_finished = true;
                }
            }
        }
    }

    pub fn load_animation_frames_from_tileset(
        tileset: &tiled::Tileset,
        from_tilemap: bool,
    ) -> HashMap<u32, Arc<Vec<Frame>>> {
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
                        if !from_tilemap {
                            frame.tile_id += tileset.first_gid;
                        }
                        frame
                    })
                    .collect();

                let tile_id = if !from_tilemap {
                    tile.id + tileset.first_gid
                } else {
                    tile.id
                };
                Some((tile_id, Arc::new(frames)))
            })
            .collect()
    }

    fn convert_tiled_frames(frame: &tiled::Frame) -> Frame {
        unsafe { std::mem::transmute(frame.clone()) }
    }
}

pub struct AnimationBuilder {
    frames: Arc<Vec<Frame>>,
    should_loop: bool,
    is_paused: bool,
}

impl AnimationBuilder {
    pub fn new(frames: Arc<Vec<Frame>>) -> Self {
        Self {
            frames,
            should_loop: false,
            is_paused: false,
        }
    }

    pub fn looping(mut self, looping: bool) -> Self {
        self.should_loop = looping;
        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.is_paused = paused;
        self
    }

    pub fn build(self) -> Animation {
        Animation {
            is_paused: self.is_paused,
            is_finished: false,
            should_loop: self.should_loop,
            current_frame: 0,
            frame_time: 0.0,
            frames: self.frames,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiles::tileset::Tileset;

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
            animation_frames_holder: Animation::load_animation_frames_from_tileset(&tileset, false),
            properties: Default::default(),
        };

        let mut animation =
            Animation::builder(Arc::clone(&tileset.animation_frames_holder[&(2 + gid)]))
                .looping(true)
                .build();

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
