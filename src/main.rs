mod game_states;
mod tiles;
mod utils;

use crate::game_states::play_state::PlayState;
use crate::game_states::state_manager::StateManager;
use glutin_window::{GlutinWindow, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

fn main() {
    let opengl_version = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Bomberman", [500, 500])
        .graphics_api(opengl_version)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new());
    let mut state_manager = StateManager::new(PlayState::build(), opengl_version);

    while let (Some(event), false) = (events.next(&mut window), state_manager.is_empty()) {
        state_manager.execute(event);
    }
}
