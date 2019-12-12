use crate::traits::game_loop_event::{Drawable, Updatable};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame {
    pub tile_id: u32,
    pub duration: u32,
}

pub trait Animatable: Updatable + Drawable {
    fn start_animation(&mut self);
    fn stop_animation(&mut self);
    fn is_animating(&self) -> bool;
    fn update_animation(&mut self, dt: f64);
}

pub fn load_animation_frames_from_tileset(tileset: &tiled::Tileset) -> HashMap<u32, Vec<Frame>> {
    tileset
        .tiles
        .iter()
        .filter_map(|tile| {
            tile.animation
                .as_ref()
                .map(convert_tiled_frames)
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
                    (tile_id, frames)
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
