use crate::asset_storage::AssetStorage;
pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;

pub trait EventHandler<T = ()> {
    fn handle_event(&mut self, asset_storage: &mut AssetStorage, event: &Event) -> T;
}

pub trait Updatable<T = ()> {
    fn update(&mut self, asset_storage: &mut AssetStorage, dt: f64) -> T;
}

pub trait Drawable {
    fn draw(&self, c: &Context, g: &mut GlGraphics);
}
