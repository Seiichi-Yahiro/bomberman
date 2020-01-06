use crate::game_states::play_state::components::*;
use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerFaceDirection};
use crate::tiles::animation::Animation;
use crate::tiles::tileset::TileId;
use crate::utils::sprite::Sprite;
use graphics::Transformed;
use legion::prelude::*;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::{ButtonEvent, ButtonState, Event, RenderEvent, UpdateEvent};
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
                let query = <(Read<ScreenPosition>, Read<CurrentTileId>, Read<Tileset>)>::query()
                    .filter(tag_value(&layer));

                for (pos, tile_id, tileset) in query.iter_immutable(world) {
                    let texture_data = tileset.0.texture_holder.get_texture_data(tile_id.0);

                    if let Some(texture_data) = texture_data {
                        if let Some(sprite) = &mut sprite {
                            sprite.update_texture_data(texture_data);
                        } else {
                            sprite = Some(Sprite::from_texture_data(texture_data));
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

pub fn create_animation_system(
    shared_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
) -> Box<dyn Schedulable> {
    SystemBuilder::new("animation_system")
        .read_resource::<Event>()
        .with_query(<(
            Write<AnimationType>,
            Write<CurrentTileId>,
            Read<DefaultTileId>,
        )>::query())
        .build(move |commands, world, event, query| {
            if let Some(update_args) = event.update_args() {
                shared_animations
                    .read()
                    .unwrap()
                    .values()
                    .for_each(|animation| {
                        animation.write().unwrap().update(update_args.dt);
                    });

                for (entity, (mut animation_type, mut current_tile_id, default_tile_id)) in
                    query.iter_entities(&mut *world)
                {
                    match &mut *animation_type {
                        AnimationType::Shared(animation) => {
                            if animation.read().unwrap().is_finished() {
                                commands.remove_component::<AnimationType>(entity);
                                current_tile_id.0 = default_tile_id.0;
                            } else {
                                current_tile_id.0 = animation.read().unwrap().get_current_tile_id();
                            }
                        }
                        AnimationType::Ownd(animation) => {
                            animation.update(update_args.dt);

                            if animation.is_finished() {
                                commands.remove_component::<AnimationType>(entity);
                                current_tile_id.0 = default_tile_id.0;
                            } else {
                                current_tile_id.0 = animation.get_current_tile_id();
                            }
                        }
                    };
                }
            }
        })
}

pub fn create_controls_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("controls_system")
        .read_resource::<Event>()
        .with_query(<(Read<Controls>, Write<MoveDirectionStack>)>::query())
        .build(move |_commands, world, event, query| {
            if let Some(button_args) = event.button_args() {
                for (controls, mut move_direction_stack) in query.iter(&mut *world) {
                    if let Some(action) = controls.0.get(&button_args.button) {
                        match action {
                            PlayerCommand::Movement(direction) => match button_args.state {
                                ButtonState::Press => {
                                    move_direction_stack.0.push(*direction);
                                }
                                ButtonState::Release => {
                                    move_direction_stack
                                        .0
                                        .iter()
                                        .position(|stored_direction| stored_direction == direction)
                                        .map(|index| move_direction_stack.0.remove(index));
                                }
                            },
                        }
                    }
                }
            }
        })
}

pub fn create_turn_player_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("turn_player_system")
        .read_resource::<Event>()
        .with_query(<(
            Read<MoveDirectionStack>,
            Read<Tileset>,
            Write<DefaultTileId>,
            Write<CurrentTileId>,
        )>::query())
        .build(move |commands, world, event, query| {
            if let Some(_button_args) = event.button_args() {
                for (
                    entity,
                    (move_direction_stack, tileset, mut default_tile_id, mut current_tile_id),
                ) in query.iter_entities(&mut *world)
                {
                    if let Some(move_direction) = move_direction_stack.0.last() {
                        let tile_id = PlayerFaceDirection::from(*move_direction)
                            .get_tile_id(&tileset.0)
                            .unwrap();

                        if default_tile_id.0 != tile_id {
                            default_tile_id.0 = tile_id;
                            current_tile_id.0 = tile_id;

                            if let Some(frames) = tileset.0.animation_frames_holder.get(&tile_id) {
                                let animation =
                                    Animation::builder(frames.clone()).looping(true).build();
                                commands.add_component(entity, AnimationType::Ownd(animation));
                            }
                        }
                    }
                }
            }
        })
}

pub fn create_move_player_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("move_player_system")
        .read_resource::<Event>()
        .with_query(<(
            Read<MoveDirectionStack>,
            Read<Speed>,
            Write<ScreenPosition>,
        )>::query())
        .build(move |_commands, world, event, query| {
            if let Some(_update_args) = event.update_args() {
                for (move_direction_stack, speed, mut screen_position) in query.iter(&mut *world) {
                    if let Some(move_direction) = move_direction_stack.0.last() {
                        let move_speed = 0.25 * speed.0;

                        *screen_position = match move_direction {
                            Direction::Up => screen_position.translate(0.0, -move_speed),
                            Direction::Down => screen_position.translate(0.0, move_speed),
                            Direction::Left => screen_position.translate(-move_speed, 0.0),
                            Direction::Right => screen_position.translate(move_speed, 0.0),
                        }
                    }
                }
            }
        })
}
