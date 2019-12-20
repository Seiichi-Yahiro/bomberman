use crate::command::{Category, Command};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::{identity, Matrix2d};
use graphics::Transformed;
use opengl_graphics::GlGraphics;

pub type Link = Box<dyn SceneNode>;

pub trait SceneNode: Updatable + Drawable {
    fn on_command(&self, command: &Command);
}

pub struct SceneTree {
    content: Link,
    children: Vec<Link>,
    transform: Matrix2d,
}

impl SceneTree {
    pub fn new(content: Link) -> SceneTree {
        SceneTree {
            content,
            children: vec![],
            transform: identity(),
        }
    }

    pub fn attach(&mut self, child: Link) {
        self.children.push(child);
    }

    fn update_children(&mut self, dt: f64) {
        self.children.iter_mut().for_each(|child| {
            child.update(dt);
        });
    }

    fn draw_children(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.children.iter().for_each(|child| {
            child.draw(transform.append_transform(self.transform), g);
        });
    }
}

impl Updatable for SceneTree {
    fn update(&mut self, dt: f64) {
        self.content.update(dt);
        self.update_children(dt);
    }
}

impl Drawable for SceneTree {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.content.draw(transform, g);
        self.draw_children(transform, g);
    }
}

impl SceneNode for SceneTree {
    fn on_command(&self, command: &Command) {
        self.children.iter().for_each(|child| {
            child.on_command(command);
        });
    }
}
