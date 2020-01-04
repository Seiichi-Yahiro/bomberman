use crate::game_states::play_state::components::*;
use crate::tiles::animation::Animation;
use crate::tiles::tileset::{TileId, Tileset};
use crate::utils::sprite::Sprite;
use graphics::Transformed;
use legion::prelude::*;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::{Event, RenderEvent, UpdateEvent};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub fn create_draw_system_fn(
    gl: Rc<RefCell<GlGraphics>>,
    number_of_layers: usize,
) -> impl FnMut(&mut World) + 'static {
    move |world: &mut World| {
        if let Some(render_args) = world.resources.get::<Event>().unwrap().render_args() {
            let ref mut g = *gl.borrow_mut();
            let c = g.draw_begin(render_args.viewport());

            let mut sprite: Option<Sprite<Texture>> = None;

            for layer in 0..number_of_layers {
                let layer = Layer(layer);
                let query = <(
                    Read<ScreenPosition>,
                    Read<CurrentTileId>,
                    Read<Arc<Tileset>>,
                )>::query()
                .filter(tag_value(&layer));

                for (pos, tile_id, tileset) in query.iter_immutable(world) {
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
                            .draw(c.transform.trans(pos.x, pos.y), g)
                    }
                }
            }

            g.draw_end();
        }
    }
}

pub fn create_update_map_position_system(
    tile_width: u32,
    tile_height: u32,
) -> Box<dyn Schedulable> {
    SystemBuilder::new("update_map_position")
        .read_resource::<Event>()
        .read_component::<MapPosition>()
        .write_component::<MapPosition>()
        .with_query(
            <Read<ScreenPosition>>::query()
                .filter(changed::<ScreenPosition>() & component::<MapPosition>()),
        )
        .build(move |_commands, world, event, query| {
            if let Some(_update_args) = event.update_args() {
                for (entity, screen_position) in query.iter_entities(&mut *world) {
                    let map_x = (screen_position.x as u32 / tile_width) * tile_width;
                    let map_y = (screen_position.y as u32 / tile_height) * tile_height;
                    let map_position = MapPosition::new(map_x, map_y);
                    if *world.get_component::<MapPosition>(entity).unwrap() != map_position {
                        *world.get_component_mut::<MapPosition>(entity).unwrap() = map_position;
                    }
                }
            }
        })
}

pub fn create_update_animation_system(
    shared_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
) -> Box<dyn Schedulable> {
    SystemBuilder::new("update_animation")
        .read_resource::<Event>()
        .with_query(<(Write<AnimationType>, Write<CurrentTileId>)>::query())
        .build(move |_commands, world, event, query| {
            if let Some(update_args) = event.update_args() {
                shared_animations
                    .read()
                    .unwrap()
                    .values()
                    .for_each(|animation| {
                        animation.write().unwrap().update(update_args.dt);
                    });

                query.par_for_each(&mut *world, |(mut animation_type, mut current_tile_id)| {
                    match &mut *animation_type {
                        AnimationType::Shared(Some(animation)) => {
                            current_tile_id.0 = animation.read().unwrap().get_current_tile_id();
                        }
                        AnimationType::Ownd(Some(animation)) => {
                            animation.update(update_args.dt);
                            current_tile_id.0 = animation.get_current_tile_id();
                        }
                        _ => {}
                    };
                });
            }
        })
}

pub fn create_exchange_animation_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("set_animation")
        .read_resource::<Event>()
        .with_query(
            <(
                Read<Arc<Tileset>>,
                Read<DefaultTileId>,
                Write<AnimationType>,
            )>::query()
            .filter(changed::<DefaultTileId>()),
        )
        .build(move |_commands, world, event, query| {
            if let Some(_update_args) = event.update_args() {
                query.par_for_each(
                    &mut *world,
                    |(tileset, default_tile_id, mut animation_type)| {
                        let mut animation = tileset
                            .animation_frames_holder
                            .get(&default_tile_id.0)
                            .cloned()
                            .map(|frames| Animation::new(frames));

                        if let Some(animation) = &mut animation {
                            if let AnimationType::Ownd(Some(old_animation)) = &*animation_type {
                                if !old_animation.is_paused() && !old_animation.is_stopped() {
                                    animation.play();
                                }
                            }
                        }

                        *animation_type = AnimationType::Ownd(animation);
                    },
                );
            }
        })
}