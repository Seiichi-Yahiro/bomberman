use crate::arenas::object_groups::{
    ArenaObjectGroup, PlayerSpawnsProperties, SoftBlockAreasProperties,
};
use crate::players::{
    MoveDirection, Player, PlayerAction, PlayerControlsMap, PlayerFaceDirection, PlayerId,
};
use engine::asset::{Object, PropertyValue, TileId, TilePosition, Tilemap, Tileset};
use engine::components::{CurrentTileId, DefaultTileId, Layer, MapPosition, ScreenPosition};
use engine::game_state::*;
use engine::legion::prelude::*;
use engine::map::Map;
use engine::scene::SceneTree;
use engine::texture::SpriteTextureDataExt;
use engine::tile::Tile;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const TILEMAP_ID: &str = "ashlands";
const PLAYER_1_TILESET_ID: &str = "player1";
const PLAYER_2_TILESET_ID: &str = "player2";

pub struct PlayState {
    map: Map,
    soft_block_entities: Vec<Entity>,
    //players: Vec<Rc<RefCell<SceneTree<Player>>>>
}

impl PlayState {
    pub fn build() -> GameStateBuilder {
        GameStateBuilderBuilder::new()
            .load_asset::<Tilemap>("assets/textures/arena_tiles/ashlands.tmx", TILEMAP_ID)
            .load_asset::<Tileset>("assets/textures/player/player1.xml", PLAYER_1_TILESET_ID)
            .load_asset::<Tileset>("assets/textures/player/player2.xml", PLAYER_2_TILESET_ID)
            .build(|data| {
                let tilemap = data.asset_storage.get_asset::<Tilemap>(TILEMAP_ID);
                let mut world = data.universe.create_world();
                let map = Map::new(Rc::clone(&tilemap), world);
                map.create_tilemap_entities();

                /*let player_spawns = Self::get_player_spawns(&tilemap);

                let player1 = Self::create_player(
                    PlayerId::Player1,
                    &data.asset_storage,
                    PLAYER_1_TILESET_ID,
                    player_spawns[&0],
                );
                let player2 = Self::create_player(
                    PlayerId::Player2,
                    &data.asset_storage,
                    PLAYER_2_TILESET_ID,
                    player_spawns[&1],
                );*/

                let mut play_state = PlayState {
                    map, //players: vec![Rc::clone(&player1), Rc::clone(&player2)],
                    soft_block_entities: vec![],
                };
                play_state.create_soft_blocks();

                //play_state.create_soft_blocks();
                //play_state.map.add_entity(1, player_spawns[&0], player1);
                //play_state.map.add_entity(1, player_spawns[&1], player2);

                Box::new(play_state)
            })
    }

    /*fn create_player(
        id: PlayerId,
        asset_storage: &AssetStorage,
        tileset_id: &str,
        position: TilePosition,
    ) -> Rc<RefCell<SceneTree<Player>>> {
        let tileset = asset_storage.get_asset::<Tileset>(tileset_id);

        let mut player = Player::new(id, tileset, Self::create_player_controls(id));

        let [x, y] = position;
        player.sprite_holder.sprite.set_position(x as f64, y as f64);
        player.sprite_holder.animation.as_mut().unwrap().play();

        let player = Rc::new(RefCell::new(player));
        let player_scene_tree = SceneTree::new(player);
        Rc::new(RefCell::new(player_scene_tree))
    }

    fn create_player_controls(player_id: PlayerId) -> PlayerControlsMap {
        let mut controls = HashMap::new();

        match player_id {
            PlayerId::Player1 => {
                controls.insert(
                    Button::Keyboard(Key::Left),
                    PlayerAction::Movement(MoveDirection::Left),
                );
                controls.insert(
                    Button::Keyboard(Key::Right),
                    PlayerAction::Movement(MoveDirection::Right),
                );
                controls.insert(
                    Button::Keyboard(Key::Up),
                    PlayerAction::Movement(MoveDirection::Up),
                );
                controls.insert(
                    Button::Keyboard(Key::Down),
                    PlayerAction::Movement(MoveDirection::Down),
                );
            }
            PlayerId::Player2 => {
                controls.insert(
                    Button::Keyboard(Key::A),
                    PlayerAction::Movement(MoveDirection::Left),
                );
                controls.insert(
                    Button::Keyboard(Key::D),
                    PlayerAction::Movement(MoveDirection::Right),
                );
                controls.insert(
                    Button::Keyboard(Key::W),
                    PlayerAction::Movement(MoveDirection::Up),
                );
                controls.insert(
                    Button::Keyboard(Key::S),
                    PlayerAction::Movement(MoveDirection::Down),
                );
            }
            PlayerId::Player3 => {}
            PlayerId::Player4 => {}
        }

        controls
    }*/

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

        let soft_block_entities = self
            .map
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(|object| {
                match object
                    .properties
                    .get(SoftBlockAreasProperties::RenderLayer.as_str())
                {
                    Some(PropertyValue::IntValue(layer_id)) => {
                        let components = (
                            MapPosition::new(object.x as u32, object.y as u32),
                            ScreenPosition::new(object.x as f64, object.y as f64),
                            DefaultTileId(object.gid),
                            CurrentTileId(object.gid),
                        );

                        let tags = (Layer(layer_id.abs() as usize),);

                        let entities = self
                            .map
                            .world
                            .borrow_mut()
                            .insert(tags, vec![components])
                            .iter()
                            .map(|entity| entity.clone())
                            .collect_vec();
                        Some(entities)
                    }
                    _ => None,
                }
            })
            .flatten()
            .collect_vec();

        self.soft_block_entities = soft_block_entities;
    }

    fn get_player_spawns(tilemap: &Tilemap) -> HashMap<i32, TilePosition> {
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
                        PropertyValue::IntValue(player_id) => {
                            Some((*player_id, [object.x as u32, object.y as u32]))
                        }
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

        /*self.players
        .iter_mut()
        .for_each(|player| player.handle_event(event));*/

        true
    }

    fn update(&mut self, _state_context: &mut StateContext<'_, '_>, dt: f64) -> bool {
        //self.map.update(dt);

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

    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.map.draw(transform, g);
    }
}
