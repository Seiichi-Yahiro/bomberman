use crate::arenas::ArenaManager;
use crate::players::PlayerManager;
use engine::game_state::*;

pub struct PlayState {
    arena_manager: ArenaManager,
    player_manager: PlayerManager,
}

impl PlayState {
    pub fn new() -> PlayState {
        let arena_manager = ArenaManager::new();

        PlayState {
            player_manager: PlayerManager::new(arena_manager.get_player_spawns()),
            arena_manager,
        }
    }
}

impl EventHandler<StateStackEvent> for PlayState {
    fn handle_event(&mut self, asset_storage: &mut AssetStorage, event: &Event) -> StateStackEvent {
        self.arena_manager.event(event);
        self.player_manager.event(event);

        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            return StateStackEvent(StateTransition::Clear, false);
        }

        StateStackEvent(StateTransition::None, true)
    }
}

impl Updatable<StateStackEvent> for PlayState {
    fn update(&mut self, asset_storage: &mut AssetStorage, dt: f64) -> StateStackEvent {
        self.arena_manager.update(dt);
        self.player_manager.update(dt);
        StateStackEvent(StateTransition::None, true)
    }
}

impl Drawable for PlayState {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_manager.draw(c, g);
        self.player_manager.draw(c, g);
    }
}

impl GameState for PlayState {}
