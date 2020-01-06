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
use piston::input::Event;
use players::{PlayerId, Players};

const TILEMAP_ID: &str = "ashlands";

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
            .build(|resources| {
                let tilemap = resources
                    .asset_storage
                    .read()
                    .unwrap()
                    .get_asset::<Tilemap>(TILEMAP_ID);

                let mut world = resources.universe.create_world();
                world.resources.insert(components::Tilemap(tilemap.clone()));

                let mut map = Map::new(tilemap.clone());
                map.create_tilemap_entities(&mut world);
                map.create_soft_blocks(&mut world);

                let mut players = Players::new();
                let player_spawns = map.get_player_spawns();
                players.create_player(
                    PlayerId::Player1,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &tilemap,
                    &mut world,
                );
                players.create_player(
                    PlayerId::Player2,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &tilemap,
                    &mut world,
                );

                let play_state = PlayState {
                    world,
                    schedule: Schedule::builder()
                        .add_system(systems::create_controls_system())
                        .add_system(systems::create_turn_player_system())
                        .add_system(systems::create_move_player_system())
                        .add_system(systems::create_collision_system())
                        .add_system(systems::create_animation_system(
                            map.tile_animations.clone(),
                        ))
                        .add_thread_local(systems::create_draw_system(
                            resources.gl.clone(),
                            tilemap.tiles.len(),
                        ))
                        //.add_thread_local(systems::create_draw_hit_box_system(resources.gl.clone()))
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
