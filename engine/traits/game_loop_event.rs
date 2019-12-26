use crate::app::AppData;
use crate::state_manager::StateContext;
pub use graphics::math::Matrix2d;
pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;

pub trait EventHandler {
    fn handle_event(&mut self, state_context: &mut StateContext, event: &Event);
}

pub trait Updatable {
    fn update(&mut self, state_context: &mut StateContext, dt: f64);
}

pub trait Drawable {
    fn draw(&self, data: &AppData, transform: Matrix2d, g: &mut GlGraphics);
}
