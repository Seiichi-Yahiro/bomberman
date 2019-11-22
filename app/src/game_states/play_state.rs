use crate::arenas::ArenaManager;
use crate::game_states::state::*;
use crate::players::PlayerManager;
use crate::traits::game_loop_event::*;
use piston::input::*;

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

impl GameLoopEvent<StateStackEvent> for PlayState {
    fn event(&mut self, event: &Event) -> StateStackEvent {
        self.arena_manager.event(event);
        self.player_manager.event(event);

        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            return StateStackEvent(StateTransition::Clear, false);
        }

        StateStackEvent(StateTransition::None, true)
    }

    fn update(&mut self, update_args: &GameLoopUpdateArgs) -> StateStackEvent {
        self.arena_manager.update(update_args);
        self.player_manager.update(update_args);
        StateStackEvent(StateTransition::None, true)
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_manager.draw(c, g);
        self.player_manager.draw(c, g);
    }
}
