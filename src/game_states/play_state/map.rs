use crate::game_states::play_state::object_groups::{
    ArenaObjectGroup, PlayerSpawnsProperties, SoftBlockAreasProperties,
};
use crate::game_states::play_state::players::PlayerId;
use crate::game_states::play_state::{components, PhysicsWorld};
use crate::tiles::animation::Animation;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::{TileId, TilePosition};
use itertools::Itertools;
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tiled::{Object, PropertyValue};

pub struct Map {
    pub tilemap: Arc<Tilemap>,
    pub tile_animations: Arc<RwLock<HashMap<TileId, Arc<RwLock<Animation>>>>>,
    pub tilemap_entities: Vec<Entity>,
    pub soft_block_entities: Vec<Entity>,
}

impl Map {
    pub fn new(tilemap: Arc<Tilemap>) -> Map {
        Map {
            tile_animations: Arc::new(RwLock::new(Self::create_shared_tile_animations(&tilemap))),
            tilemap,
            tilemap_entities: vec![],
            soft_block_entities: vec![],
        }
    }

    fn create_shared_tile_animations(tilemap: &Tilemap) -> HashMap<TileId, Arc<RwLock<Animation>>> {
        let mut used_tile_ids = tilemap.get_used_tile_ids();

        tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .map(|object| object.gid)
            .for_each(|tile_id| {
                used_tile_ids.insert(tile_id);
            });

        used_tile_ids
            .iter()
            .filter_map(|tile_id| {
                let frames = tilemap
                    .tileset
                    .animation_frames_holder
                    .get(tile_id)
                    .cloned()?;

                let animation = Animation::builder(frames)
                    .paused(false)
                    .looping(true)
                    .build();

                Some((*tile_id, Arc::new(RwLock::new(animation))))
            })
            .collect()
    }

    pub fn create_tilemap_entities(&mut self, world: &mut World, physics_world: &mut PhysicsWorld) {
        self.tilemap_entities = self
            .tilemap
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(layer_index, layer)| {
                layer
                    .iter()
                    .map(|(&[x, y], &tile_id)| {
                        let entity = self.create_tilemap_entity(
                            world,
                            components::EntityType::HardBlock,
                            layer_index,
                            tile_id,
                        );

                        self.try_adding_physical_component(
                            world,
                            physics_world,
                            entity,
                            tile_id,
                            x as f64,
                            y as f64,
                        );
                        self.try_adding_shared_animation_component(world, entity, tile_id);

                        entity
                    })
                    .collect_vec()
            })
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

        let create_entity = |object: &Object| match object
            .properties
            .get(SoftBlockAreasProperties::RenderLayer.as_str())
        {
            Some(PropertyValue::IntValue(layer_id)) => {
                let x = object.x.abs() as f64;
                let y = object.y.abs() as f64;

                let entity = self.create_tilemap_entity(
                    world,
                    components::EntityType::SoftBlock,
                    *layer_id as usize,
                    object.gid,
                );

                self.try_adding_physical_component(world, physics_world, entity, object.gid, x, y);
                self.try_adding_shared_animation_component(world, entity, object.gid);

                Some(entity)
            }
            _ => None,
        };

        self.soft_block_entities = self
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(create_entity)
            .collect_vec();
    }

    fn create_tilemap_entity(
        &self,
        world: &mut World,
        entity_type: components::EntityType,
        layer: usize,
        tile_id: TileId,
    ) -> Entity {
        let tags = (components::Layer(layer), entity_type);
        let components = (
            components::DefaultTileId(tile_id),
            components::CurrentTileId(tile_id),
            components::Tileset(self.tilemap.tileset.clone()),
        );

        *world
            .insert(tags, vec![components])
            .first()
            .unwrap()
    }

    fn try_adding_physical_component(
        &self,
        world: &mut World,
        physics_world: &mut PhysicsWorld,
        entity: Entity,
        tile_id: TileId,
        x: f64,
        y: f64,
    ) {
        if let Some(&[hx, hy, w, h]) = self.tilemap.tileset.hit_boxes.get(&tile_id) {
            let half_tile_width = self.tilemap.tile_width as f64 / 2.0;
            let half_tile_height = self.tilemap.tile_height as f64 / 2.0;

            let body = RigidBodyDesc::new()
                .translation(Vector2::new(x + half_tile_width, y + half_tile_height))
                .status(BodyStatus::Static)
                .gravity_enabled(false)
                .user_data(entity)
                .build();

            let body_handle = physics_world.bodies.insert(body);

            let collider = ColliderDesc::new(ShapeHandle::new(Cuboid::new(Vector2::new(
                w / 2.0,
                h / 2.0,
            ))))
            .translation(Vector2::new(
                hx - half_tile_width + w / 2.0,
                hy - half_tile_height + h / 2.0,
            ))
            .user_data(entity)
            .build(BodyPartHandle(body_handle, 0));

            let collider_handle = physics_world.colliders.insert(collider);

            world.add_component(entity, components::BodyHandle(body_handle));
            world.add_component(entity, components::ColliderHandle(collider_handle));
        } else {
            world.add_component(entity, components::ScreenPosition([x, y]));
        }
    }

    fn try_adding_shared_animation_component(
        &self,
        world: &mut World,
        entity: Entity,
        tile_id: TileId,
    ) {
        if let Some(animation) = self.tile_animations.read().unwrap().get(&tile_id).cloned() {
            world.add_component(entity, components::AnimationType::Shared(animation));
        }
    }

    pub fn get_player_spawns(&self) -> HashMap<PlayerId, TilePosition> {
        self.tilemap
            .object_groups
            .get(ArenaObjectGroup::PlayerSpawns.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter_map(|object| {
                object
                    .properties
                    .get(PlayerSpawnsProperties::PlayerId.as_str())
                    .and_then(|property_value| match property_value {
                        PropertyValue::IntValue(player_id) => Some((
                            PlayerId::from(player_id.abs() as u32),
                            [object.x.abs() as u32, object.y.abs() as u32],
                        )),
                        _ => None,
                    })
            })
            .collect()
    }
}
