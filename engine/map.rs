use crate::animation::Animation;
use crate::app::AppData;
use crate::components::{
    AnimationType, CurrentTileId, DefaultTileId, DeltaTime, Layer, MapPosition, ScreenPosition,
};
use crate::sprite::Sprite;
use crate::state_manager::StateContext;
use crate::tilemap::Tilemap;
use crate::tileset::{TileId, Tileset};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use graphics::Transformed;
use itertools::Itertools;
use legion::prelude::*;
use opengl_graphics::{GlGraphics, Texture};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Map {
    pub tilemap: Arc<Tilemap>,
    pub tile_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
    pub world: RefCell<World>,
    animation_schedule: Schedule,
}

impl Map {
    pub fn new(tilemap: Arc<Tilemap>, world: World) -> Map {
        let tile_animations = tilemap
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
            .collect();

        let tile_animations = Arc::new(RwLock::new(tile_animations));

        Map {
            animation_schedule: Schedule::builder()
                .add_system(AnimationType::create_exchange_animation_system())
                .add_system(AnimationType::create_update_animation_system(Arc::clone(
                    &tile_animations,
                )))
                .build(),
            tilemap,
            tile_animations,
            world: RefCell::new(world),
        }
    }

    pub fn create_tilemap_entities(&self) {
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

                self.world
                    .borrow_mut()
                    .insert((Layer(layer_index),), components);
            });
    }
}

impl Updatable for Map {
    fn update(&mut self, state_context: &mut StateContext, dt: f64) {
        self.world.borrow_mut().resources.insert(DeltaTime(dt));
        self.animation_schedule
            .execute(&mut *self.world.borrow_mut());
    }
}

impl Drawable for Map {
    fn draw(&self, data: &AppData, transform: Matrix2d, g: &mut GlGraphics) {
        let mut sprite: Option<Sprite<Texture>> = None;

        for layer in 0..self.tilemap.tiles.len() {
            let layer = Layer(layer);
            let query = <(
                Read<ScreenPosition>,
                Read<CurrentTileId>,
                Read<Arc<Tileset>>,
            )>::query()
            .filter(tag_value(&layer));

            for (pos, tile_id, tileset) in query.iter(&mut self.world.borrow_mut()) {
                let texture_data = tileset.texture_holder.get_texture_data(tile_id.0);

                if let Some(texture_data) = texture_data {
                    if let Some(sprite) = &mut sprite {
                        sprite.update_texture_data(texture_data);
                    } else {
                        sprite = Some(Sprite::from_texture_data(texture_data.clone()));
                    }

                    sprite
                        .as_ref()
                        .unwrap()
                        .draw(transform.trans(pos.x, pos.y), g)
                }
            }
        }
    }
}
