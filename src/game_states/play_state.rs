mod components;
mod map;
mod object_groups;
mod players;
mod systems;

use crate::game_states::game_state_builder::{GameStateBuilder, GameStateBuilderBuilder};
use crate::game_states::state_manager::GameState;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::Tileset;
use legion::schedule::Schedule;
use legion::world::World;
use map::Map;
use nalgebra::{RealField, Vector2};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use piston::input::Event;
use players::{PlayerId, Players};

const TILEMAP_ID: &str = "ashlands";

pub struct PhysicsWorld<N: RealField = f64> {
    mechanical_world: DefaultMechanicalWorld<N>,
    geometrical_world: DefaultGeometricalWorld<N>,
    bodies: DefaultBodySet<N>,
    colliders: DefaultColliderSet<N>,
    joint_constraints: DefaultJointConstraintSet<N>,
    force_generators: DefaultForceGeneratorSet<N>,
}

impl PhysicsWorld<f64> {
    pub fn new() -> PhysicsWorld<f64> {
        PhysicsWorld {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
        }
    }
}

pub struct PlayState {
    world: World,
    schedule: Schedule,
    map: Map,
    players: Players,
}

impl PlayState {
    pub fn build() -> GameStateBuilder {
        GameStateBuilderBuilder::new()
            .load_asset::<Tilemap>("assets/textures/arena_tiles/ashlands.tmx", TILEMAP_ID)
            .load_asset::<Tileset>(
                "assets/textures/player/player1.xml",
                PlayerId::Player1.as_str(),
            )
            .load_asset::<Tileset>(
                "assets/textures/player/player2.xml",
                PlayerId::Player2.as_str(),
            )
            .load_asset::<Tileset>("assets/textures/bomb/bomb.xml", "bomb")
            .build(|resources| {
                let tilemap = resources
                    .asset_storage
                    .read()
                    .unwrap()
                    .get_asset::<Tilemap>(TILEMAP_ID);

                let mut physics_world = PhysicsWorld::<f64>::new();

                let mut world = resources.universe.create_world();
                world
                    .resources
                    .insert(components::AssetStorage(resources.asset_storage.clone()));
                world.resources.insert(components::Tilemap(tilemap.clone()));

                let mut map = Map::new(tilemap.clone());
                map.create_tilemap_entities(&mut world, &mut physics_world);
                map.create_soft_blocks(&mut world, &mut physics_world);

                let mut players = Players::new();
                let player_spawns = map.get_player_spawns();
                players.create_player(
                    PlayerId::Player1,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &tilemap,
                    &mut world,
                    &mut physics_world,
                );
                players.create_player(
                    PlayerId::Player2,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &tilemap,
                    &mut world,
                    &mut physics_world,
                );

                world.resources.insert(physics_world);
                let play_state = PlayState {
                    world,
                    schedule: Schedule::builder()
                        .add_system(systems::create_controls_system())
                        .add_system(systems::create_collision_events_system())
                        .flush()
                        .add_system(systems::create_spawn_bomb_system())
                        .add_system(systems::create_update_bomb_collision_status_system())
                        .add_system(systems::create_turn_player_system())
                        .add_system(systems::create_move_player_system())
                        .add_system(systems::create_update_physics_world_system())
                        .add_system(systems::create_clear_collision_events_system())
                        .add_system(systems::create_animation_system(
                            map.tile_animations.clone(),
                        ))
                        .add_thread_local(systems::create_draw_system(
                            resources.gl.clone(),
                            tilemap.tiles.len() + 1,
                        ))
                        .add_thread_local(systems::create_draw_hit_box_system(resources.gl.clone()))
                        .build(),
                    map,
                    players,
                };

                Box::new(play_state)
            })
    }
}

impl GameState for PlayState {
    fn execute(&mut self, event: Event) -> bool {
        self.world.resources.insert(event);
        self.schedule.execute(&mut self.world);
        true
    }
}
