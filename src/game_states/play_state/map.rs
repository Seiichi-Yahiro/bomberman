use crate::game_states::play_state::components;
use crate::game_states::play_state::object_groups::{
    ArenaObjectGroup, PlayerSpawnsProperties, SoftBlockAreasProperties,
};
use crate::game_states::play_state::players::PlayerId;
use crate::tiles::animation::Animation;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::{TileId, TilePosition};
use itertools::Itertools;
use legion::entity::Entity;
use legion::world::World;
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
        tilemap
            .get_used_tile_ids()
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

    pub fn create_tilemap_entities(&mut self, world: &mut World) {
        self.tilemap_entities = self
            .tilemap
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(layer_index, layer)| {
                layer
                    .iter()
                    .map(|(&[x, y], tile_id)| {
                        let entity = world
                            .insert(
                                (
                                    components::Layer(layer_index),
                                    components::XMapPosition(x / self.tilemap.tile_width),
                                    components::YMapPosition(y / self.tilemap.tile_height),
                                ),
                                vec![(
                                    components::ScreenPosition([x as f64, y as f64]),
                                    components::HitBox([
                                        0.0,
                                        0.0,
                                        self.tilemap.tile_width as f64,
                                        self.tilemap.tile_height as f64,
                                    ]),
                                    components::DefaultTileId(*tile_id),
                                    components::CurrentTileId(*tile_id),
                                    components::Tileset(self.tilemap.tileset.clone()),
                                )],
                            )
                            .first()
                            .unwrap()
                            .clone();

                        if let Some(animation) =
                            self.tile_animations.read().unwrap().get(tile_id).cloned()
                        {
                            world.add_component(
                                entity,
                                components::AnimationType::Shared(animation),
                            );
                        }

                        entity
                    })
                    .collect_vec()
            })
            .collect_vec();
    }

    pub fn create_soft_blocks(&mut self, world: &mut World) {
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
                let x = object.x.abs();
                let y = object.y.abs();

                let tags = (
                    components::Layer(*layer_id as usize),
                    components::XMapPosition(x as u32 / self.tilemap.tile_width),
                    components::YMapPosition(y as u32 / self.tilemap.tile_height),
                );

                let components = (
                    components::ScreenPosition([x as f64, y as f64]),
                    components::HitBox([
                        0.0,
                        0.0,
                        self.tilemap.tile_width as f64,
                        self.tilemap.tile_height as f64,
                    ]),
                    components::DefaultTileId(object.gid),
                    components::CurrentTileId(object.gid),
                    components::Tileset(self.tilemap.tileset.clone()),
                );

                Some(
                    world
                        .insert(tags, vec![components])
                        .first()
                        .unwrap()
                        .clone(),
                )
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
