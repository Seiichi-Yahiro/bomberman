mod app;
pub mod character;
pub mod map;
mod state_manager;
mod texture_holder;
mod traits;
mod utils;

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

pub mod texture {
    pub use super::texture_holder::*;
}
