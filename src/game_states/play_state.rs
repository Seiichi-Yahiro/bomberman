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
                let tilemap = data.asset_storage.get_asset::<Tilemap>(TILEMAP_ID);
                let mut world = data.universe.create_world();

                let player_spawns = Self::get_player_spawns(&tilemap);

                let player1 = Player::create_player(
                    PlayerId::Player1,
                    &player_spawns,
                    &data.asset_storage,
                    &mut world,
                );
                let player2 = Player::create_player(
                    PlayerId::Player2,
                    &player_spawns,
                    &data.asset_storage,
                    &mut world,
                );

                let map = Map::new(Rc::clone(&tilemap), world);
                map.create_tilemap_entities();

                let mut play_state = PlayState {
                    map,
                    soft_block_entities: vec![],
                    players: vec![player1, player2],
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

        Player::handle_event(
            &mut *self.map.world.borrow_mut(),
            &state_context.data.asset_storage,
            event,
        );

        true
    }

    fn update(&mut self, state_context: &mut StateContext<'_, '_>, dt: f64) -> bool {
        Player::update(
            &mut *self.map.world.borrow_mut(),
            &state_context.data.asset_storage,
            dt,
        );
        self.map.update(state_context, dt);

        /*self.players
        .iter()
        .filter_map(|player| {
            let move_direction = player.get_current_move_direction()?;
            let tile_id = player.get_tile_id_for_move_direction(move_direction)?;
            let speed = match move_direction {
                MoveDirection::Up => [0.0, -1.0],
                MoveDirection::Down => [0.0, 1.0],
                MoveDirection::Left => [-1.0, 0.0],
                MoveDirection::Right => [1.0, 0.0],
            };

            Some((player.tile_uuid, speed, tile_id))
        })
        .collect::<Vec<(TileUuid, [f64; 2], TileId)>>()
        .into_iter()
        .for_each(|(id, speed, tile_id)| {
            if let Some(player_tile) = self.map.tiles[1].get_mut_tile_by_id(id) {
                let [vx, vy] = speed;
                let (x, y) = player_tile.sprite_holder.sprite.get_position();
                player_tile
                    .sprite_holder
                    .sprite
                    .set_position(x + vx * dt * 32.0 * 2.0, y + vy * dt * 32.0 * 2.0);
                player_tile.sprite_holder.update_tile_id(tile_id);
            }
        });*/

        true
    }

    fn draw(&self, data: &AppData, transform: Matrix2d, g: &mut GlGraphics) {
        self.map.draw(data, transform, g);
    }
}
