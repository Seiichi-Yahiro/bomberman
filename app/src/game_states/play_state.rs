use crate::arenas::{ArenaManager, Arenas};
use crate::game_states::state::*;
use crate::generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet;
use crate::traits::game_loop_event::*;
use crate::traits::FromRON;
use piston::input::*;
use std::path::Path;

pub struct PlayState {
    arena_manager: ArenaManager,
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

        PlayState { arena_manager }
    }
}

impl GameLoopEvent<StateStackEvent> for PlayState {
    fn event(&mut self, event: &Event) -> StateStackEvent {
        self.arena_manager.event(event);

        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            return StateStackEvent(StateTransition::Clear, false);
        }

        StateStackEvent(StateTransition::None, true)
    }

    fn update(&mut self, _dt: f64) -> StateStackEvent {
        StateStackEvent(StateTransition::None, true)
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_manager.draw(c, g);
    }
}
