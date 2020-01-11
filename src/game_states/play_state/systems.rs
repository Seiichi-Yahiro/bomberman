use crate::game_states::play_state::components::*;
use crate::game_states::play_state::players::{Direction, PlayerCommand, PlayerFaceDirection};
use crate::game_states::play_state::PhysicsWorld;
use crate::tiles::animation::Animation;
use crate::tiles::tileset::TileId;
use crate::utils::sprite::Sprite;
use graphics::Transformed;
use itertools::Itertools;
use legion::prelude::*;
use nalgebra::Vector2;
use ncollide2d::narrow_phase::ContactEvent;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::object::{Body, BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
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
        .read_resource::<PhysicsWorld>()
        .with_query(<(Read<ScreenPosition>, Read<CurrentTileId>, Read<Tileset>)>::query())
        .with_query(<(Read<BodyHandle>, Read<CurrentTileId>, Read<Tileset>)>::query())
        .build_thread_local(move |_commands, world, (event, physics_world), query| {
            if let Some(render_args) = event.render_args() {
                let ref mut g = *gl.borrow_mut();
                let c = g.draw_begin(render_args.viewport());

                let mut sprite: Option<Sprite<Texture>> = None;

                for layer in 0..number_of_layers {
                    let layer = Layer(layer);

                    query
                        .0
                        .clone()
                        .filter(tag_value(&layer))
                        .iter_immutable(&*world)
                        .for_each(|(pos, tile_id, tileset)| {
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
                        });

                    query
                        .1
                        .clone()
                        .filter(tag_value(&layer))
                        .iter_immutable(&*world)
                        .for_each(|(body, tile_id, tileset)| {
                            let p: &PhysicsWorld = &*physics_world;
                            let body = p.bodies.rigid_body(body.0).unwrap();
                            let pos = body.position().translation.vector.data;

                            let texture_data = tileset.0.texture_holder.get_texture_data(tile_id.0);

                            if let Some(texture_data) = texture_data {
                                let [_, _, w, h] = texture_data.src_rect;

                                if let Some(sprite) = &mut sprite {
                                    sprite.update_texture_data(texture_data);
                                } else {
                                    sprite = Some(Sprite::from_texture_data(texture_data));
                                }

                                sprite
                                    .as_ref()
                                    .unwrap()
                                    .draw(c.transform.trans(pos[0] - w / 2.0, pos[1] - h / 2.0), g)
                            }
                        });
                }

                g.draw_end();
            }
        })
}

#[cfg(debug_assertions)]
pub fn create_draw_hit_box_system(gl: Rc<RefCell<GlGraphics>>) -> Box<dyn Runnable> {
    SystemBuilder::new("draw_hit_box_system")
        .read_resource::<Event>()
        .read_resource::<PhysicsWorld>()
        .with_query(<Read<ColliderHandle>>::query())
        .build_thread_local(move |_commands, world, (event, physics_world), query| {
            if let Some(render_args) = event.render_args() {
                let ref mut g = *gl.borrow_mut();
                let c = g.draw_begin(render_args.viewport());

                query.iter_immutable(&*world).for_each(|collider| {
                    let p: &PhysicsWorld = &*physics_world;
                    let [x, y, w, h] = {
                        let collider = p.colliders.get(collider.0).unwrap();
                        let size = collider.shape_handle().local_aabb().extents().data;
                        let pos = collider.position().translation.vector.data;
                        [
                            pos[0] - size[0] / 2.0,
                            pos[1] - size[1] / 2.0,
                            size[0],
                            size[1],
                        ]
                    };

                    let color = [0.0, 1.0, 0.0, 0.7];
                    let radius = 0.5;
                    graphics::line(color, radius, [x, y, x + w, y], c.transform, g);
                    graphics::line(color, radius, [x + w, y, x + w, y + h], c.transform, g);
                    graphics::line(color, radius, [x, y + h, x + w, y + h], c.transform, g);
                    graphics::line(color, radius, [x, y, x, y + h], c.transform, g);
                });

                g.draw_end();
            }
        })
}

pub fn create_update_physics_world_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("update_physics_world_system")
        .read_resource::<Event>()
        .write_resource::<PhysicsWorld>()
        .build(move |_commands, _world, (event, physics_world), _query| {
            if let Some(_update_args) = event.update_args() {
                let p: &mut PhysicsWorld = &mut *physics_world;
                p.mechanical_world.step(
                    &mut p.geometrical_world,
                    &mut p.bodies,
                    &mut p.colliders,
                    &mut p.joint_constraints,
                    &mut p.force_generators,
                );
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
        .build(move |commands, world, event, query| {
            if let Some(button_args) = event.button_args() {
                for (entity, (controls, mut move_direction_stack)) in
                    query.iter_entities(&mut *world)
                {
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
                            PlayerCommand::Bomb => {
                                if button_args.state == ButtonState::Press {
                                    commands.insert((), vec![(SpawnBomb(entity),)]);
                                }
                            }
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
        .write_resource::<PhysicsWorld>()
        .with_query(<(
            Read<MoveDirectionStack>,
            Read<MovementSpeed>,
            Read<BodyHandle>,
        )>::query())
        .build(move |_commands, world, (event, physics_world), query| {
            if let Some(_update_args) = event.update_args() {
                query
                    .iter(&mut *world)
                    .for_each(|(move_direction_stack, movement_speed, body)| {
                        if let Some(move_direction) = move_direction_stack.0.last() {
                            let p: &mut PhysicsWorld = &mut *physics_world;

                            let move_speed = movement_speed.0;

                            let force = match move_direction {
                                Direction::Up => Vector2::new(0.0, -move_speed),
                                Direction::Down => Vector2::new(0.0, move_speed),
                                Direction::Left => Vector2::new(-move_speed, 0.0),
                                Direction::Right => Vector2::new(move_speed, 0.0),
                            };

                            p.bodies.rigid_body_mut(body.0).unwrap().apply_force(
                                0,
                                &Force2::linear(force),
                                ForceType::Impulse,
                                true,
                            );
                        }
                    })
            }
        })
}

pub fn create_spawn_bomb_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("spawn_bomb_system")
        .read_resource::<Event>()
        .read_resource::<AssetStorage>()
        .write_resource::<PhysicsWorld>()
        .read_component::<BodyHandle>()
        .with_query(<Read<SpawnBomb>>::query())
        .build(
            move |commands, world, (event, asset_storage, physics_world), query| {
                if let Some(_update_args) = event.update_args() {
                    for (entity, spawn_bomb) in query.iter_entities_immutable(&*world) {
                        let tileset: Arc<crate::tiles::tileset::Tileset> = asset_storage
                            .0
                            .read()
                            .unwrap()
                            .get_asset::<crate::tiles::tileset::Tileset>("bomb");

                        let tile_id = 1;

                        let p: &mut PhysicsWorld = &mut *physics_world;

                        let [x, y] = {
                            let body = world.get_component::<BodyHandle>(spawn_bomb.0).unwrap();
                            let body = p.bodies.rigid_body(body.0).unwrap();
                            let pos = body.position().translation.vector.data;
                            [pos[0], pos[1]]
                        };

                        let (body_handle, collider_handle) = {
                            let [hx, hy, w, h] = tileset.hit_boxes[&tile_id];

                            let body = RigidBodyDesc::new()
                                .status(BodyStatus::Disabled)
                                .linear_damping(5.0)
                                .mass(1.0)
                                .translation(Vector2::new(x, y))
                                .gravity_enabled(false)
                                .build();
                            let body_handle = p.bodies.insert(body);

                            let collider = ColliderDesc::new(ShapeHandle::new(Cuboid::new(
                                Vector2::new(w / 2.0, h / 2.0),
                            )))
                            /*.translation(Vector2::new(
                                hx - half_tile_width + w / 2.0,
                                hy - half_tile_height + h / 2.0,
                            ))*/
                            .user_data(EntityType::Bomb)
                            .build(BodyPartHandle(body_handle, 0));

                            let collider_handle = p.colliders.insert(collider);

                            (body_handle, collider_handle)
                        };

                        let animation =
                            Animation::builder(tileset.animation_frames_holder[&tile_id].clone())
                                .looping(true)
                                .build();

                        let tags = (Layer(1), EntityType::Bomb);
                        let components = (
                            Tileset(tileset),
                            DefaultTileId(tile_id),
                            CurrentTileId(tile_id),
                            AnimationType::Ownd(animation),
                            BodyHandle(body_handle),
                            ColliderHandle(collider_handle),
                        );

                        commands.insert(tags, vec![components]);
                        commands.delete(entity);
                    }
                }
            },
        )
}

pub fn create_update_bomb_collision_status_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("update_bomb_collision_status_system")
        .read_resource::<Event>()
        .write_resource::<PhysicsWorld>()
        .build(move |_commands, _world, (event, physics_world), _query| {
            if let Some(_update_args) = event.update_args() {
                let p: &mut PhysicsWorld = &mut *physics_world;
                p.geometrical_world
                    .contact_events()
                    .iter()
                    .filter_map(|contact_event| match contact_event {
                        ContactEvent::Started(h1, h2) => None,
                        ContactEvent::Stopped(h1, h2) => {
                            let collider1 = p.colliders.get(*h1).unwrap();
                            let collider2 = p.colliders.get(*h2).unwrap();
                            match (
                                collider1
                                    .user_data()
                                    .and_then(|it| it.downcast_ref::<EntityType>()),
                                collider2
                                    .user_data()
                                    .and_then(|it| it.downcast_ref::<EntityType>()),
                            ) {
                                (Some(EntityType::Bomb), Some(EntityType::Player)) => {
                                    Some(collider1.body())
                                }
                                (Some(EntityType::Player), Some(EntityType::Bomb)) => {
                                    Some(collider2.body())
                                }
                                _ => None,
                            }
                        }
                    })
                    .collect_vec()
                    .into_iter()
                    .for_each(|bomb_handle| {
                        p.bodies
                            .rigid_body_mut(bomb_handle)
                            .unwrap()
                            .set_status(BodyStatus::Static);
                    });
            }
        })
}
