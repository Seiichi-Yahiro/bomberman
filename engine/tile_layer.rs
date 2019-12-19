use crate::command::Command;
use crate::scene::SceneNode;
use crate::sprite_holder::SpriteHolder;
use crate::tileset::TilePosition;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use std::collections::HashMap;

pub struct TileLayer {
    tiles: HashMap<TilePosition, SpriteHolder>, // TODO use ref
}

impl Updatable for TileLayer {
    fn update(&mut self, dt: f64) {}
}

impl Drawable for TileLayer {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.tiles.iter().for_each(|([x, y], sprite_holder)| {
            sprite_holder.draw(transform.trans(*x as f64, *y as f64), g);
        });
    }
}

impl SceneNode for TileLayer {
    fn get_category(&self) -> u32 {
        unimplemented!()
    }

    fn on_command(&self, command: &Command) {
        unimplemented!()
    }
}
