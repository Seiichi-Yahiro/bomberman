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

pub fn create_draw_system(
    gl: Rc<RefCell<GlGraphics>>,
    number_of_layers: usize,
) -> Box<dyn Runnable> {
    SystemBuilder::new("draw_system")
        .read_resource::<Event>()
        .with_query(<(Read<ScreenPosition>, Read<CurrentTileId>, Read<Tileset>)>::query())
        .build_thread_local(move |_commands, world, event, query| {
            if let Some(render_args) = event.render_args() {
                let ref mut g = *gl.borrow_mut();
                let c = g.draw_begin(render_args.viewport());

                let mut sprite: Option<Sprite<Texture>> = None;

                for layer in 0..number_of_layers {
                    let layer = Layer(layer);
                    let query = query.clone().filter(tag_value(&layer));

                    for (pos, tile_id, tileset) in query.iter_immutable(&*world) {
                        let texture_data = tileset.0.texture_holder.get_texture_data(tile_id.0);

                        if let Some(texture_data) = texture_data {
                            if let Some(sprite) = &mut sprite {
                                sprite.update_texture_data(texture_data);
                            } else {
                                sprite = Some(Sprite::from_texture_data(texture_data));
                            }

                            let [x, y] = pos.0;

                            sprite.as_ref().unwrap().draw(c.transform.trans(x, y), g)
                        }
                    }
                }

                g.draw_end();
            }
        })
}

#[cfg(debug_assertions)]
pub fn create_draw_hit_box_system(gl: Rc<RefCell<GlGraphics>>) -> Box<dyn Runnable> {
    SystemBuilder::new("draw_hit_box_system")
        .read_resource::<Event>()
        .with_query(<(Read<ScreenPosition>, Read<HitBox>)>::query())
        .build_thread_local(move |_commands, world, event, query| {
            if let Some(render_args) = event.render_args() {
                let ref mut g = *gl.borrow_mut();
                let c = g.draw_begin(render_args.viewport());

                let layer = Layer(1);
                for (screen_position, hit_box) in query
                    .clone()
                    .filter(tag_value(&layer))
                    .iter_immutable(&*world)
                {
                    let [x, y] = screen_position.0;
                    let [w, h] = hit_box.0;

                    let color = [0.0, 1.0, 0.0, 0.7];
                    let radius = 0.5;
                    graphics::line(color, radius, [x, y, x + w, y], c.transform, g);
                    graphics::line(color, radius, [x + w, y, x + w, y + h], c.transform, g);
                    graphics::line(color, radius, [x, y + h, x + w, y + h], c.transform, g);
                    graphics::line(color, radius, [x, y, x, y + h], c.transform, g);
                }

                g.draw_end();
            }
        })
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
            Write<PreviousScreenPosition>,
        )>::query())
        .build(move |_commands, world, event, query| {
            if let Some(_update_args) = event.update_args() {
                for (
                    move_direction_stack,
                    speed,
                    mut screen_position,
                    mut previous_screen_position,
                ) in query.iter(&mut *world)
                {
                    if let Some(move_direction) = move_direction_stack.0.last() {
                        let move_speed = 0.25 * speed.0;

                        previous_screen_position.0 = screen_position.0;

                        *screen_position = match move_direction {
                            Direction::Up => screen_position.translate([0.0, -move_speed]),
                            Direction::Down => screen_position.translate([0.0, move_speed]),
                            Direction::Left => screen_position.translate([-move_speed, 0.0]),
                            Direction::Right => screen_position.translate([move_speed, 0.0]),
                        }
                    }
                }
            }
        })
}

pub fn create_collision_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("collision_system")
        .read_resource::<Event>()
        .read_resource::<Tilemap>()
        .with_query(<(
            Read<HitBox>,
            Write<ScreenPosition>,
            Write<PreviousScreenPosition>,
        )>::query())
        .with_query(
            <(Read<ScreenPosition>, Read<HitBox>)>::query()
                .filter(!component::<PreviousScreenPosition>()),
        )
        .build(
            move |_commands, world, (event, tilemap), (query, compare_query)| {
                if let Some(_update_args) = event.update_args() {
                    for (hit_box, mut screen_position, mut previous_screen_position) in
                        query.iter(&mut *world)
                    {
                        let [map_x, map_y] = {
                            let [x, y] = screen_position.0;
                            [
                                x as u32 / tilemap.0.tile_width,
                                y as u32 / tilemap.0.tile_height,
                            ]
                        };

                        let collision = [
                            [map_x, map_y],
                            [map_x + 1, map_y],
                            [map_x, map_y + 1],
                            [map_x + 1, map_y + 1],
                        ]
                        .iter()
                        .any(|&[map_x, map_y]| {
                            let x_map_position = XMapPosition(map_x);
                            let y_map_position = YMapPosition(map_y);
                            let layer = Layer(1);

                            let [x, y] = screen_position.0;
                            let [w, h] = hit_box.0;

                            for (other_screen_position, other_hit_box) in compare_query
                                .clone()
                                .filter(
                                    tag_value(&x_map_position)
                                        & tag_value(&y_map_position)
                                        & tag_value(&layer),
                                )
                                .iter_immutable(&*world)
                            {
                                let [ox, oy] = other_screen_position.0;
                                let [ow, oh] = other_hit_box.0;

                                if x < ox + ow && x + w > ox && y < oy + oh && y + h > oy {
                                    return true;
                                }
                            }

                            false
                        });

                        if collision {
                            screen_position.0 = previous_screen_position.0;
                        }
                    }
                }
            },
        )
}
