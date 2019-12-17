//use crate::arenas::ArenaManager;
//use crate::players::PlayerManager;
use crate::arenas::object_groups::{ArenaObjectGroup, SoftBlockAreasProperties};
use engine::asset::Tilemap;
use engine::game_state::*;
use engine::map::Map;
use engine::tile::Tile;
use std::rc::Rc;

const MAP_ID: &str = "ashlands";

pub struct PlayState {
    map: Map,
}

impl PlayState {
    pub fn build() -> GameStateBuilder {
        GameStateBuilder {
            prepare: vec![Box::new(|asset_storage| {
                asset_storage.load_asset_from_file::<Tilemap>(
                    std::path::Path::new("assets/textures/arena_tiles/ashlands.tmx"),
                    MAP_ID,
                );
            })],
            create: Box::new(|asset_storage| {
                let mut play_state = PlayState {
                    map: Map::from_tilemap(asset_storage.get_asset::<Tilemap>(MAP_ID)),
                };
                play_state.create_soft_blocks();

                Box::new(play_state)
            }),
        }
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
            self.map
                .tiles
                .get_mut(event.layer)
                .and_then(|layer| layer.insert(event));
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
