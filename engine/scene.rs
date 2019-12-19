use crate::command::{Category, Command};
use graphics::Context;
use opengl_graphics::GlGraphics;

pub type Link = Box<dyn SceneNode>;

pub trait SceneNode {
    fn update(&mut self, dt: f64);
    fn draw(&self, c: &Context, g: &mut GlGraphics);

    /// Commands are applied to categories
    /// 0 is predefined for SceneGraphs
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

    fn draw_children(&self, c: &Context, g: &mut GlGraphics) {
        self.children.iter().for_each(|child| {
            child.draw(c, g);
        });
    }
}

impl SceneNode for SceneGraph {
    fn update(&mut self, dt: f64) {
        self.update_children(dt);
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.draw_children(c, g);
    }

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
