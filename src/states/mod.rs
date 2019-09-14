mod load;
mod menu;

pub use menu::Menu;

pub mod prelude {
    pub use super::load::{Assets, LoadState, LoadStateBuilder, LoadableState};
}
