mod inital;
mod load;
mod menu;

pub use inital::InitialState;
pub use menu::Menu;

pub mod prelude {
    pub use super::load::{
        get_asset_handle, with_asset, AssetHandles, LoadState, LoadStateBuilder, LoadableState,
    };
}
