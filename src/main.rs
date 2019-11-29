mod arenas;
mod game_states;
mod players;
mod utils;

use engine::prelude::*;
use game_states::play_state::PlayState;

fn main() {
    App::new(
        WindowSettings::new("Bomberman", [500, 500]),
        EventSettings::new(),
        OpenGL::V4_5,
    )
    .run(Box::new(PlayState::new()));
}
