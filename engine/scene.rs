/*
use crate::asset_storage::AssetStorage;
use crate::command::Command;
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::{identity, Matrix2d};
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use std::cell::RefCell;
use std::rc::Rc;

pub type SceneNodeLink = Rc<RefCell<dyn SceneNode>>;

pub trait SceneNode: Updatable + Drawable {
    fn on_command(&self, command: &Command);
}

#[derive(Clone)]
pub struct SceneTree<T: SceneNode> {
    pub content: Rc<RefCell<T>>,
    children: Vec<SceneNodeLink>,
    transform: Matrix2d,
}

impl<T: SceneNode> SceneTree<T> {
    pub fn new(content: Rc<RefCell<T>>) -> SceneTree<T> {
        SceneTree {
            content,
            children: vec![],
            transform: identity(),
        }
    }

    pub fn attach(&mut self, child: SceneNodeLink) {
        self.children.push(child);
    }

    fn update_children(&mut self, dt: f64) {
        self.children.iter_mut().for_each(|child| {
            child.borrow_mut().update(dt);
        });
    }

    fn draw_children(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.children.iter().for_each(|child| {
            child
                .borrow()
                .draw(transform.append_transform(self.transform), g);
        });
    }
}

impl<T: SceneNode> Updatable for SceneTree<T> {
    fn update(&mut self, dt: f64) {
        self.content.borrow_mut().update(dt);
        self.update_children(dt);
    }
}

impl<T: SceneNode> Drawable for SceneTree<T> {
    fn draw(&self, asset_storage: &AssetStorage, transform: Matrix2d, g: &mut GlGraphics) {
        self.content.borrow().draw(asset_storage, transform, g);
        self.draw_children(transform, g);
    }
}

impl<T: SceneNode> SceneNode for SceneTree<T> {
    fn on_command(&self, command: &Command) {
        self.children.iter().for_each(|child| {
            child.borrow().on_command(command);
        });
    }
}
*/
