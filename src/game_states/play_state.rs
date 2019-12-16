//use crate::arenas::ArenaManager;
//use crate::players::PlayerManager;
use engine::asset::Tilemap;
use engine::game_state::*;
use engine::map::Map;

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
                Box::new(PlayState {
                    map: Map::from_tilemap(asset_storage.get_asset::<Tilemap>(MAP_ID)),
                })
            }),
        }
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
