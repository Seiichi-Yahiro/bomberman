use crate::arenas::{ArenaController, ArenaView, Arenas};
use crate::game_states::state::*;
use crate::generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet;
use crate::traits::controller::Controller;
use crate::traits::view::View;
use crate::traits::FromRON;
use piston::input::*;
use std::path::Path;

pub struct PlayState {
    arena_controller: ArenaController,
    arena_view: ArenaView,
}

impl PlayState {
    pub fn new() -> PlayState {
        let arena_controller = {
            let Arenas(arenas) =
                Arenas::load_from_ron_file(Path::new("app/assets/arenas/arenas.ron"));

            ArenaController {
                arena: arenas[0].init(),
                spritesheet: ArenaTilesSpriteSheet::new(),
            }
        };

        PlayState {
            arena_controller,
            arena_view: ArenaView::new(),
        }
    }
}

impl State for PlayState {
    fn event(&mut self, event: &Event) -> StateStackEvent {
        self.arena_controller.event(event);

        if let Some(Button::Keyboard(Key::Escape)) = event.press_args() {
            return StateStackEvent(StateTransition::Clear, false);
        }

        StateStackEvent(StateTransition::None, true)
    }

    fn update(&mut self, dt: f64) -> StateStackEvent {
        StateStackEvent(StateTransition::None, true)
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.arena_view.draw(&self.arena_controller, c, g);
    }
}
