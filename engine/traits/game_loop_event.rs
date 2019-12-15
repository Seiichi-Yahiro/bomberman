use crate::world::World;
pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;

pub trait EventHandler<T = ()> {
    fn handle_event(&mut self, world: &mut World, event: &Event) -> T;
}

pub trait Updatable<T = ()> {
    fn update(&mut self, world: &mut World, dt: f64) -> T;
}

pub trait Drawable {
    fn draw(&self, world: &World, c: &Context, g: &mut GlGraphics);
}
