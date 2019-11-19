pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;

pub trait GameLoopEvent<T: Default> {
    fn event(&mut self, _event: &Event) -> T {
        T::default()
    }

    fn update(&mut self, _dt: f64) -> T {
        T::default()
    }

    fn draw(&self, _c: &Context, _g: &mut GlGraphics) {}
}
