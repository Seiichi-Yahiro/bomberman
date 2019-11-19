use crate::arenas::{ArenaManager, Arenas};
use crate::game_states::state::*;
use crate::generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet;
use crate::players::PlayerManager;
use crate::traits::game_loop_event::*;
use crate::traits::FromRON;
use piston::input::*;
use std::path::Path;

pub struct PlayState {
    arena_manager: ArenaManager,
    player_manager: PlayerManager,
}

impl PlayState {
    pub fn new() -> PlayState {
        let arena_manager = {
            let Arenas(arenas) =
                Arenas::load_from_ron_file(Path::new("app/assets/arenas/arenas.ron"));

            ArenaManager {
                arena: arenas[0].init(),
                spritesheet: ArenaTilesSpriteSheet::new(),
            }
        };

        let player_manager = {
            let player_spawns = arena_manager.arena.get_player_spawns();
            PlayerManager::new(player_spawns)
        };

        PlayState {
            arena_manager,
            player_manager,
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

    fn update(&mut self, dt: f64) -> StateStackEvent {
        self.arena_manager.update(dt);
        self.player_manager.update(dt);
        StateStackEvent(StateTransition::None, true)
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_manager.draw(c, g);
        self.player_manager.draw(c, g);
    }
}
