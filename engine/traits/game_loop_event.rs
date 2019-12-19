pub use graphics::math::Matrix2d;
pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event);
}

pub trait Updatable {
    fn update(&mut self, dt: f64);
}

pub trait Drawable {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics);
}
