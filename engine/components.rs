use crate::animation::Animation;
use crate::asset_storage::AssetStorage;
use crate::tileset::{TileId, Tileset};
use legion::prelude::*;
use legion::schedule::Schedulable;
use legion::system::SystemBuilder;
use legion::world::World;
use piston::input::{Button, ButtonState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeltaTime(pub f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenPosition {
    pub x: f64,
    pub y: f64,
}

impl ScreenPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layer(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurrentTileId(pub TileId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DefaultTileId(pub TileId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TilesetType<'s> {
    Tilemap,
    Tileset(&'s str),
}

#[derive(Clone, Debug)]
pub enum AnimationType {
    Shared(Option<Arc<RwLock<Animation>>>),
    Ownd(Option<Animation>),
}

impl AnimationType {
    pub fn create_update_animation_system(
        shared_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
    ) -> Box<dyn Schedulable> {
        SystemBuilder::new("update_animation")
            .read_resource::<DeltaTime>()
            .with_query(<(Write<AnimationType>, Write<CurrentTileId>)>::query())
            .build(move |commands, world, dt, query| {
                shared_animations
                    .read()
                    .unwrap()
                    .values()
                    .for_each(|animation| {
                        animation.write().unwrap().update(dt.0);
                    });

                for (mut animation_type, mut current_tile_id) in query.iter(&mut *world) {
                    match &mut *animation_type {
                        AnimationType::Shared(Some(animation)) => {
                            current_tile_id.0 = animation.read().unwrap().get_current_tile_id();
                        }
                        AnimationType::Ownd(Some(animation)) => {
                            animation.update(dt.0);
                            current_tile_id.0 = animation.get_current_tile_id();
                        }
                        _ => {}
                    }
                }
            })
    }

    pub fn create_exchange_animation_system(
        asset_storage: Rc<RefCell<AssetStorage>>,
    ) -> Box<dyn Runnable> {
        SystemBuilder::new("set_animation")
            .with_query(
                <(Read<TilesetType>, Read<DefaultTileId>, Write<AnimationType>)>::query()
                    .filter(changed::<DefaultTileId>()),
            )
            .build_thread_local(move |commands, world, resources, query| {
                for (tileset_type, default_tile_id, mut animation_type) in query.iter(&mut *world) {
                    if let TilesetType::Tileset(id) = *tileset_type {
                        let tileset = asset_storage.borrow().get_asset::<Tileset>(id);

                        let mut animation = tileset
                            .animation_frames_holder
                            .get(&default_tile_id.0)
                            .cloned()
                            .map(|frames| Animation::new(frames));

                        if let AnimationType::Ownd(Some(old_animation)) = &*animation_type {
                            if !old_animation.is_paused() && !old_animation.is_stopped() {
                                if let Some(animation) = &mut animation {
                                    animation.play();
                                }
                            }
                        }

                        *animation_type = AnimationType::Ownd(animation);
                    }
                }
            })
    }
}

pub type Command = Box<dyn Fn(&mut World) + Send + Sync>;
pub type CommandFactory = Box<dyn Fn(ButtonState) -> Command + Send + Sync>;
pub type ControlsMap = HashMap<Button, CommandFactory>;

pub struct Controls(pub ControlsMap);
