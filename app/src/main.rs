extern crate ai_behavior;
extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate sprite;
extern crate tiled;

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
use traits::game_loop_event::GameLoopEvent;

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

    while let Some(event) = events.next(&mut window) {
        state_manager.event(&event);

        if let Some(update_args) = event.update_args() {
            state_manager.update(update_args.dt);
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
