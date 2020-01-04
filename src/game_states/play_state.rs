mod components;
mod map;
mod object_groups;
mod players;
mod systems;

use crate::game_states::game_state_builder::{GameStateBuilder, GameStateBuilderBuilder};
use crate::game_states::play_state::map::Map;
use crate::game_states::play_state::players::{PlayerId, Players};
use crate::game_states::play_state::systems::*;
use crate::game_states::state_manager::GameState;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::Tileset;
use legion::schedule::Schedule;
use legion::world::World;
use piston::input::Event;

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

                let mut map = Map::new(tilemap.clone());
                map.create_tilemap_entities(&mut world);
                map.create_soft_blocks(&mut world);

                let mut players = Players::new();
                let player_spawns = map.get_player_spawns();
                players.create_player(
                    PlayerId::Player1,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &mut world,
                );
                players.create_player(
                    PlayerId::Player2,
                    &player_spawns,
                    &resources.asset_storage.read().unwrap(),
                    &mut world,
                );

                let play_state = PlayState {
                    world,
                    schedule: Schedule::builder()
                        //.add_system(Player::create_turn_player_system())
                        //.add_system(Player::create_move_player_system())
                        //.add_system(create_update_map_position_system(tilemap.tile_width, tilemap.tile_height, ))
                        //.add_system(create_exchange_animation_system())
                        .add_system(create_controls_system())
                        .add_system(create_turn_player_system())
                        //.add_system(create_update_animation_system(map.tile_animations.clone()))
                        .add_thread_local_fn(create_draw_system_fn(
                            resources.gl.clone(),
                            tilemap.tiles.len(),
                        ))
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
