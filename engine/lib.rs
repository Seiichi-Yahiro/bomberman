pub mod animation;
mod app;
mod asset_storage;
pub mod command;
pub mod components;
mod game_state_builder;
pub mod map;
pub mod scene;
mod sprite_holder;
mod state_manager;
mod texture_holder;
mod tilemap;
mod tileset;
mod traits;
mod utils;

pub use legion;

pub mod prelude {
    pub use super::app::App;
    pub use opengl_graphics::OpenGL;
    pub use piston::{event_loop::*, input::*, WindowSettings};
}

pub mod game_state {
    pub use super::app::AppData;
    pub use super::asset_storage::AssetStorage;
    pub use super::game_state_builder::*;
    pub use super::state_manager::*;
    pub use super::traits::game_loop_event::*;
    pub use piston::input::*;
}

pub mod texture {
    pub use super::texture_holder::*;
}

pub mod sprite {
    pub use super::sprite_holder::*;
}

pub mod asset {
    pub use super::asset_storage::*;
    pub use super::tilemap::*;
    pub use super::tileset::*;
    pub use tiled::Object;
    pub use tiled::PropertyValue;
}
