mod arenas;
mod game_states;
mod players;
mod traits;
mod utils;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use traits::game_loop_event::*;

fn main() {
    let opengl = OpenGL::V4_5;
    let (width, height) = (500, 500);

    let mut window: Window = WindowSettings::new("Bomberman", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut state_manager =
        game_states::state::StateManager::new(Box::new(game_states::play_state::PlayState::new()));
    let mut key_state = KeyState::new();

    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            key_state.insert(key, true);
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            key_state.insert(key, false);
        }

        state_manager.event(&event);

        if let Some(update_args) = event.update_args() {
            let game_loop_update_args = GameLoopUpdateArgs {
                dt: update_args.dt,
                key_state: &key_state,
            };
            state_manager.update(&game_loop_update_args);
        }

        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                state_manager.draw(&c, g);
            })
        }

        if state_manager.is_empty() {
            break;
        }
    }
}
