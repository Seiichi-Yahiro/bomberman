pub use graphics::Context;
pub use opengl_graphics::GlGraphics;
pub use piston::input::Event;
use piston::input::Key;
use std::collections::HashMap;

pub struct KeyState {
    map: HashMap<Key, bool>,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Key, val: bool) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: &Key) -> bool {
        *self.map.get(key).unwrap_or(&false)
    }
}

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
