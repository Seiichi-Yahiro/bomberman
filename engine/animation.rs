use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

#[derive(Debug)]
pub struct Animation {
    current_tile_id: u32,
    default_tile_id: u32,
    animation_time: f64,
    animation_length: f64,
    frames: Vec<Frame>,
    pub is_animating: bool,
}

impl Animation {
    pub fn new(tile_id: u32, frames: Vec<Frame>) -> Animation {
        Animation {
            current_tile_id: tile_id,
            default_tile_id: tile_id,
            animation_time: 0.0,
            animation_length: frames
                .iter()
                .fold(0.0, |acc, item| acc + item.duration as f64),
            frames,
            is_animating: false,
        }
    }

    pub fn start_animation(&mut self) {
        self.is_animating = true;
        self.animation_time = 0.0;
    }

    pub fn stop_animation(&mut self) {
        self.is_animating = false;
        self.current_tile_id = self.default_tile_id;
    }

    pub fn update(&mut self, dt: f64) {
        if !self.is_animating {
            return;
        }

        self.animation_time = (self.animation_time + dt * 1000.0) % self.animation_length;

        let frame_index = self
            .frames
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

#[derive(Debug)]
pub struct Animations {
    animations: HashMap<u32, Animation>,
}

impl Animations {
    pub fn from_tileset(tileset: &tiled::Tileset) -> Animations {
        let animations = tileset
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
                        (tile_id, Animation::new(tile_id, frames))
                    })
            })
            .collect();

        Animations { animations }
    }

    fn convert_tiled_frames(frames: &Vec<tiled::Frame>) -> Vec<Frame> {
        frames
            .iter()
            .map(|frame| unsafe { std::mem::transmute(frame.clone()) })
            .collect()
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

        let gid = 1;

        let tileset = tiled::parse_tileset(tileset_str.as_bytes(), gid).unwrap();
        let mut animations = Animations::from_tileset(&tileset);
        let animation = animations.animations.get_mut(&(2 + gid)).unwrap();

        let mut result = Vec::new();

        animation.start_animation();
        result.push(animation.current_tile_id);

        animation.update(100.0 / 1000.0);
        result.push(animation.current_tile_id);

        animation.update(100.0 / 1000.0);
        result.push(animation.current_tile_id);

        animation.update(200.0 / 1000.0);
        result.push(animation.current_tile_id);

        animation.update(200.0 / 1000.0);
        result.push(animation.current_tile_id);

        animation.update(200.0 / 1000.0);
        result.push(animation.current_tile_id);

        assert_eq!(
            result.as_ref(),
            [2 + gid, 0 + gid, 2 + gid, 1 + gid, 2 + gid, 0 + gid]
        );
    }
}
