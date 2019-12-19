use crate::command::{Category, Command};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

pub type Link = Box<dyn SceneNode>;

pub trait SceneNode: Updatable + Drawable {
    /// Commands are applied to categories
    /// 0 is predefined as no category
    fn get_category(&self) -> Category;

    fn on_command(&self, command: &Command);
}

pub struct SceneGraph {
    children: Vec<Link>,
}

impl SceneGraph {
    fn attach(&mut self, child: Link) {
        self.children.push(child);
    }

    fn update_children(&mut self, dt: f64) {
        self.children.iter_mut().for_each(|child| {
            child.update(dt);
        });
    }

    fn draw_children(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.children.iter().for_each(|child| {
            child.draw(transform, g);
        });
    }
}

impl Updatable for SceneGraph {
    fn update(&mut self, dt: f64) {
        self.update_children(dt);
    }
}

impl Drawable for SceneGraph {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.draw_children(transform, g);
    }
}

impl SceneNode for SceneGraph {
    fn get_category(&self) -> Category {
        0
    }

    fn on_command(&self, command: &Command) {
        if self.get_category() == command.get_category() {
            command.execute(self);
        }

        self.children.iter().for_each(|child| {
            child.on_command(command);
        });
    }
}
