mod app;
mod state_manager;
mod traits;

pub mod prelude {
    pub use super::app::App;
    pub use opengl_graphics::OpenGL;
    pub use piston::{event_loop::*, input::*, WindowSettings};
}

pub mod game_state {
    pub use super::state_manager::*;
    pub use super::traits::game_loop_event::*;
    pub use piston::input::*;
}
