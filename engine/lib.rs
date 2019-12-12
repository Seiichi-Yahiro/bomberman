pub mod animation;
mod app;
mod asset_storage;
pub mod character;
pub mod map;
mod sprite;
mod state_manager;
mod texture_holder;
mod tilemap;
mod tileset;
mod traits;
mod utils;

pub mod prelude {
    pub use super::app::App;
    pub use opengl_graphics::OpenGL;
    pub use piston::{event_loop::*, input::*, WindowSettings};
}

pub mod game_state {
    pub use super::asset_storage::AssetStorage;
    pub use super::state_manager::*;
    pub use super::traits::game_loop_event::*;
    pub use piston::input::*;
}

pub mod texture {
    pub use super::texture_holder::*;
}
