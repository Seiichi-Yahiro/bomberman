//use crate::arenas::ArenaManager;
//use crate::players::PlayerManager;
use crate::arenas::object_groups::{ArenaObjectGroup, SoftBlockAreasProperties};
use crate::players::{Player, PlayerFaceDirection};
use engine::asset::{Tilemap, Tileset};
use engine::game_state::*;
use engine::map::Map;
use engine::tile::Tile;
use std::rc::Rc;

const TILEMAP_ID: &str = "ashlands";
const PLAYER_1_TILESET_ID: &str = "player1";
const PLAYER_2_TILESET_ID: &str = "player2";

pub struct PlayState {
    map: Map,
    player1: Player,
    player2: Player,
}

impl PlayState {
    pub fn build() -> GameStateBuilder {
        GameStateBuilder {
            prepare: vec![Box::new(|asset_storage| {
                asset_storage.load_asset_from_file::<Tilemap>(
                    std::path::Path::new("assets/textures/arena_tiles/ashlands.tmx"),
                    TILEMAP_ID,
                );

                asset_storage.load_asset_from_file::<Tileset>(
                    std::path::Path::new("assets/textures/player/player1.xml"),
                    PLAYER_1_TILESET_ID,
                );

                asset_storage.load_asset_from_file::<Tileset>(
                    std::path::Path::new("assets/textures/player/player2.xml"),
                    PLAYER_2_TILESET_ID,
                );
            })],
            create: Box::new(|asset_storage| {
                let (player1, player1_tile) =
                    Self::create_player(asset_storage, PLAYER_1_TILESET_ID, 32.0, 32.0);
                let (player2, player2_tile) =
                    Self::create_player(asset_storage, PLAYER_2_TILESET_ID, 416.0, 352.0);

                let mut play_state = PlayState {
                    map: Map::from_tilemap(asset_storage.get_asset::<Tilemap>(TILEMAP_ID)),
                    player1,
                    player2,
                };
                play_state.create_soft_blocks();

                play_state.map.tiles[1].insert(player1_tile);
                play_state.map.tiles[1].insert(player2_tile);

                Box::new(play_state)
            }),
        }
    }

    fn create_player(
        asset_storage: &AssetStorage,
        tileset_id: &str,
        x: f64,
        y: f64,
    ) -> (Player, Tile) {
        let tileset = asset_storage.get_asset::<Tileset>(tileset_id);
        let face_directions_to_tile_ids = Player::map_face_directions_to_tile_ids(&tileset);

        let mut player_tile = Tile::from_tileset(
            tileset,
            face_directions_to_tile_ids[&PlayerFaceDirection::Down],
            1,
        )
        .unwrap();

        player_tile.sprite_holder.sprite.set_position(x, y);
        player_tile.sprite_holder.animation.as_mut().unwrap().play();

        let player = Player::new(player_tile.id, face_directions_to_tile_ids);

        (player, player_tile)
    }

    fn create_soft_blocks(&mut self) {
        let should_spawn_soft_block = |soft_block: &&tiled::Object| -> bool {
            soft_block
                .properties
                .get(SoftBlockAreasProperties::SpawnChance.as_str())
                .map(|property_value| match property_value {
                    tiled::PropertyValue::FloatValue(spawn_chance) => {
                        rand::random::<f32>() <= *spawn_chance
                    }
                    _ => false,
                })
                .unwrap_or(false)
        };

        let soft_blocks: Vec<Tile> = self
            .map
            .tilemap
            .object_groups
            .get(ArenaObjectGroup::SoftBlockAreas.as_str())
            .iter()
            .flat_map(|objects| objects.iter())
            .filter(should_spawn_soft_block)
            .filter_map(|object| {
                object
                    .properties
                    .get(SoftBlockAreasProperties::RenderLayer.as_str())
                    .and_then(|property_value| match property_value {
                        tiled::PropertyValue::IntValue(layer_id) => {
                            let tileset = Rc::clone(&self.map.tilemap.tileset);
                            let mut event =
                                Tile::from_tileset(tileset, object.gid, layer_id.abs() as usize)?;
                            event
                                .sprite_holder
                                .sprite
                                .set_position(object.x as f64, object.y as f64);
                            Some(event)
                        }
                        _ => None,
                    })
            })
            .collect();

        soft_blocks.into_iter().for_each(|event| {
            if let Some(layer) = self.map.tiles.get_mut(event.layer) {
                layer.insert(event);
            }
        });
    }
}

impl GameState for PlayState {
    fn handle_event(&mut self, state_context: &mut StateContext<'_, '_>, event: &Event) -> bool {
        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            (state_context.request_state_transition)(StateTransition::Clear);
            return false;
        }

        true
    }

    fn update(&mut self, _state_context: &mut StateContext<'_, '_>, dt: f64) -> bool {
        self.map.update(dt);
        true
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.map.draw(c, g);
    }
}
