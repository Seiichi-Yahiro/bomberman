//use crate::arenas::ArenaManager;
//use crate::players::PlayerManager;
use engine::asset::Tilemap;
use engine::game_state::*;
use engine::map::Map;

pub struct PlayState {}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {}
    }
}

impl EventHandler<StateStackEvent> for PlayState {
    fn handle_event(&mut self, _world: &mut World, event: &Event) -> StateStackEvent {
        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            return StateStackEvent(StateTransition::Clear, false);
        }

        StateStackEvent(StateTransition::None, true)
    }
}

impl Updatable<StateStackEvent> for PlayState {
    fn update(&mut self, world: &mut World, dt: f64) -> StateStackEvent {
        world.get_mut_map().update(world, dt);
        StateStackEvent(StateTransition::None, true)
    }
}

impl Drawable for PlayState {
    fn draw(&self, world: &World, c: &Context, g: &mut GlGraphics) {
        world.get_map().draw(world, c, g);
    }
}

impl GameState for PlayState {
    fn on_create(&mut self, world: &mut World) {
        world.asset_storage.load_asset_from_file::<Tilemap>(
            std::path::Path::new("assets/textures/arena_tiles/ashlands.tmx"),
            "map_id".to_string(),
        );

        world.set_map(Map::from_tilemap(
            world
                .asset_storage
                .get_asset::<Tilemap>("map_id".to_string()),
        ))
    }
}
