use crate::game_states::play_state::object_groups::{
    ArenaObjectGroup, PlayerSpawnsProperties, SoftBlockAreasProperties,
};
use crate::game_states::play_state::players::PlayerId;
use crate::game_states::play_state::{components, PhysicsWorld};
use crate::tiles::animation::{Animation, AnimationBuilder};
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::TileId;
use itertools::Itertools;
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tiled::{Object, ObjectShape, PropertyValue};

pub struct Map {
    pub tilemap: Arc<Tilemap>,
    pub hard_block_entities: Vec<Entity>,
    pub soft_block_entities: Vec<Entity>,
}

impl Map {
    pub fn new(tilemap: Arc<Tilemap>) -> Map {
        Map {
            tilemap,
            hard_block_entities: vec![],
            soft_block_entities: vec![],
        }
    }

    fn create_entity(
        &self,
        world: &mut World,
        physics_world: &mut PhysicsWorld,
        object: &Object,
    ) -> Option<Entity> {
        let render_layer = object.properties.get("render_layer");

        match (render_layer, &object.shape) {
            (Some(PropertyValue::IntValue(layer)), ObjectShape::Rect { width, height }) => {
                let body_x = object.x as f64;
                let body_y = object.y as f64;
                let half_body_width = *width as f64 / 2.0;
                let half_body_height = *height as f64 / 2.0;

                let entity = {
                    let tags = (components::EntityType::SoftBlock,);
                    let components = (
                        components::Layer(*layer as usize),
                        components::DefaultTileId(object.gid),
                        components::CurrentTileId(object.gid),
                        components::Tileset(self.tilemap.tileset.clone()),
                    );

                    *world.insert(tags, vec![components]).first().unwrap()
                };

                let body_handle = {
                    let body = RigidBodyDesc::new()
                        .translation(Vector2::new(
                            body_x + half_body_width,
                            body_y + half_body_height,
                        ))
                        .status(BodyStatus::Static)
                        .gravity_enabled(false)
                        .user_data(entity)
                        .build();

                    physics_world.bodies.insert(body)
                };

                world.add_component(entity, components::BodyHandle(body_handle));

                if let Some(&[x, y, w, h]) = self.tilemap.tileset.hit_boxes.get(&object.gid) {
                    let half_hit_box_width = w / 2.0;
                    let half_hit_box_height = h / 2.0;

                    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
                        half_hit_box_width,
                        half_hit_box_height,
                    )));

                    let collider = ColliderDesc::new(shape)
                        .translation(Vector2::new(
                            x - half_body_width + half_hit_box_width,
                            y - half_body_height + half_hit_box_height,
                        ))
                        .user_data(entity)
                        .build(BodyPartHandle(body_handle, 0));

                    let collider_handle = physics_world.colliders.insert(collider);

                    world.add_component(entity, components::ColliderHandle(collider_handle));
                }

                if let Some(frames) = self
                    .tilemap
                    .tileset
                    .animation_frames_holder
                    .get(&object.gid)
                    .cloned()
                {
                    let animation = AnimationBuilder::new(frames)
                        .looping(true)
                        .paused(false)
                        .build();
                    world.add_component(entity, components::Animation(animation));
                }

                Some(entity)
            }
            _ => None,
        }
    }

    pub fn create_hard_blocks(&mut self, world: &mut World, physics_world: &mut PhysicsWorld) {
        self.hard_block_entities = self
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::HardBlocks.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter_map(|object| self.create_entity(world, physics_world, object))
            .collect_vec();
    }

    pub fn create_soft_blocks(&mut self, world: &mut World, physics_world: &mut PhysicsWorld) {
        let should_spawn_soft_block = |soft_block: &&Object| -> bool {
            soft_block
                .properties
                .get(SoftBlockAreasProperties::SpawnChance.as_str())
                .map(|property_value| match property_value {
                    PropertyValue::FloatValue(spawn_chance) => {
                        rand::random::<f32>() <= *spawn_chance
                    }
                    _ => false,
                })
                .unwrap_or(false)
        };

        self.soft_block_entities = self
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(|object| self.create_entity(world, physics_world, object))
            .collect_vec();
    }

    pub fn get_player_spawns(&self) -> HashMap<PlayerId, [f64; 4]> {
        self.tilemap
            .object_groups
            .get(ArenaObjectGroup::PlayerSpawns.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter_map(|object| {
                object
                    .properties
                    .get(PlayerSpawnsProperties::PlayerId.as_str())
                    .and_then(|property_value| match (property_value, &object.shape) {
                        (
                            PropertyValue::IntValue(player_id),
                            ObjectShape::Rect { width, height },
                        ) => Some((
                            PlayerId::from(player_id.abs() as u32),
                            [
                                object.x as f64,
                                object.y as f64,
                                *width as f64,
                                *height as f64,
                            ],
                        )),
                        _ => None,
                    })
            })
            .collect()
    }
}
