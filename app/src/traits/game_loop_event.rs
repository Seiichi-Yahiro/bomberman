pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;
use piston::input::Key;
use std::collections::HashMap;

pub type KeyState = HashMap<Key, bool>;

pub struct GameLoopUpdateArgs<'a> {
    pub dt: f64,
    pub key_state: &'a KeyState,
}

pub trait GameLoopEvent<T: Default> {
    fn event(&mut self, _event: &Event) -> T {
        T::default()
    }

    fn update(&mut self, _update_args: &GameLoopUpdateArgs) -> T {
        T::default()
    }

    fn draw(&self, _c: &Context, _g: &mut GlGraphics) {}
}
