use crate::game_states::play_state::components::*;
use crate::tiles::animation::Animation;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::TileId;
use itertools::Itertools;
use legion::world::World;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Map {
    pub tilemap: Arc<Tilemap>,
    pub tile_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
}

impl Map {
    pub fn new(tilemap: Arc<Tilemap>) -> Map {
        Map {
            tile_animations: Arc::new(RwLock::new(Self::create_shared_tile_animations(&tilemap))),
            tilemap,
        }
    }

    fn create_shared_tile_animations(tilemap: &Tilemap) -> HashMap<TileId, Arc<RwLock<Animation>>> {
        tilemap
            .get_used_tile_ids()
            .iter()
            .filter_map(|tile_id| {
                let frames = tilemap
                    .tileset
                    .animation_frames_holder
                    .get(tile_id)
                    .cloned()?;

                let mut animation = Animation::new(frames);
                animation.play();

                Some((*tile_id, Arc::new(RwLock::new(animation))))
            })
            .collect()
    }

    pub fn create_tilemap_entities(&self, world: &mut World) {
        self.tilemap
            .tiles
            .iter()
            .enumerate()
            .for_each(|(layer_index, layer)| {
                let components = layer
                    .iter()
                    .map(|(&[x, y], tile_id)| {
                        (
                            MapPosition::new(x, y),
                            ScreenPosition::new(x as f64, y as f64),
                            DefaultTileId(*tile_id),
                            CurrentTileId(*tile_id),
                            Arc::clone(&self.tilemap.tileset),
                            AnimationType::Shared(
                                self.tile_animations.read().unwrap().get(tile_id).cloned(),
                            ),
                        )
                    })
                    .collect_vec();

                world.insert((Layer(layer_index),), components);
            });
    }
}
