use crate::arenas::object_groups::{
    ArenaObjectGroup, PlayerSpawnsProperties, SoftBlockAreasProperties,
};
use crate::players::{
    MoveDirection, MoveDirectionStack, Player, PlayerAction, PlayerFaceDirection, PlayerId,
};
use engine::asset::{Object, PropertyValue, TilePosition, Tilemap, Tileset};
use engine::components::{
    Controls, CurrentTileId, DefaultTileId, Layer, MapPosition, ScreenPosition, TilesetType,
};
use engine::game_state::input::*;
use engine::game_state::*;
use engine::legion::prelude::*;
use engine::map::Map;
use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;

const TILEMAP_ID: &str = "ashlands";

pub struct PlayState {
    map: Map,
    soft_block_entities: Vec<Entity>,
    players: Vec<Entity>,
    schedule: Schedule,
}

impl PlayState {
    pub fn build() -> GameStateBuilder {
        GameStateBuilderBuilder::new()
            .load_asset::<Tilemap>("assets/textures/arena_tiles/ashlands.tmx", TILEMAP_ID)
            .load_asset::<Tileset>(
                "assets/textures/player/player1.xml",
                PlayerId::Player1.to_str(),
            )
            .load_asset::<Tileset>(
                "assets/textures/player/player2.xml",
                PlayerId::Player2.to_str(),
            )
            .build(|data| {
                let tilemap = data.asset_storage.borrow().get_asset::<Tilemap>(TILEMAP_ID);
                let mut world = data.universe.create_world();

                let player_spawns = Self::get_player_spawns(&tilemap);

                let player1 = Player::create_player(
                    PlayerId::Player1,
                    &player_spawns,
                    &data.asset_storage.borrow(),
                    &mut world,
                );
                let player2 = Player::create_player(
                    PlayerId::Player2,
                    &player_spawns,
                    &data.asset_storage.borrow(),
                    &mut world,
                );

                let map = Map::new(Rc::clone(&tilemap), Rc::clone(&data.asset_storage), world);
                map.create_tilemap_entities();

                let mut play_state = PlayState {
                    map,
                    soft_block_entities: vec![],
                    players: vec![player1, player2],
                    schedule: Schedule::builder()
                        .add_thread_local(Player::create_turn_player_system(Rc::clone(
                            &data.asset_storage,
                        )))
                        .build(),
                };

                play_state.create_soft_blocks();

                Box::new(play_state)
            })
    }

    fn create_soft_blocks(&mut self) {
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

        let create_components_grouped_by_layer = |object: &Object| match object
            .properties
            .get(SoftBlockAreasProperties::RenderLayer.as_str())
        {
            Some(PropertyValue::IntValue(layer_id)) => {
                let x = object.x.abs();
                let y = object.y.abs();

                let components = (
                    MapPosition::new(x as u32, y as u32),
                    ScreenPosition::new(x as f64, y as f64),
                    DefaultTileId(object.gid),
                    CurrentTileId(object.gid),
                    TilesetType::Tilemap,
                );

                Some((*layer_id, components))
            }
            _ => None,
        };

        self.soft_block_entities = self
            .map
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(create_components_grouped_by_layer)
            .into_group_map()
            .into_iter()
            .map(|(layer_id, components)| {
                let tags = (Layer(layer_id.abs() as usize),);

                self.map
                    .world
                    .borrow_mut()
                    .insert(tags, components)
                    .iter()
                    .map(|entity| entity.clone())
                    .collect_vec()
            })
            .flatten()
            .collect_vec();
    }

    fn get_player_spawns(tilemap: &Tilemap) -> HashMap<PlayerId, TilePosition> {
        tilemap
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

impl GameState for PlayState {
    fn handle_event(&mut self, state_context: &mut StateContext<'_, '_>, event: &Event) -> bool {
        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            (state_context.request_state_transition)(StateTransition::Clear);
            return false;
        }

        Player::handle_event(&mut *self.map.world.borrow_mut(), event);

        true
    }

    fn update(&mut self, state_context: &mut StateContext<'_, '_>, dt: f64) -> bool {
        Player::update(
            &mut *self.map.world.borrow_mut(),
            &state_context.data.asset_storage.borrow(),
            dt,
        );
        self.schedule.execute(&mut self.map.world.borrow_mut());
        self.map.update(state_context, dt);

        true
    }

    fn draw(&self, data: &AppData, transform: Matrix2d, g: &mut GlGraphics) {
        self.map.draw(data, transform, g);
    }
}
