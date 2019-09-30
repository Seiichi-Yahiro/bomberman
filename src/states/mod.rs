mod game;
mod initial;
mod load;
mod menu;

pub use game::Game;
pub use initial::InitialState;
pub use menu::Menu;

pub mod prelude {
    pub use super::load::{
        get_asset_handle, get_dynamic_asset_handle, with_asset, with_dynamic_asset, AssetHandles,
        LoadState, LoadStateBuilder, LoadableState,
    };
}
