extern crate ai_behavior;
extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate sprite;

mod arena;
mod generated;
mod traits;

use std::path::Path;
use traits::controller::*;
use traits::view::*;
use traits::FromRON;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V4_5;
    let (width, height) = (500, 500);

    let mut window: Window = WindowSettings::new("Bomberman", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut arena_controller = {
        let arena::Arenas(arenas) =
            arena::Arenas::load_from_ron_file(Path::new("app/assets/arenas/arenas.ron"));

        arena::ArenaController {
            arena: arenas[0].init(),
            spritesheet: generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet::new(),
        }
    };
    let arena_view = arena::ArenaView {};

    while let Some(event) = events.next(&mut window) {
        arena_controller.event(&event);

        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                arena_view.draw(&arena_controller, &c, g);
            })
        }
    }
}
