/*
use crate::scene::SceneNode;

pub type Category = u32;

pub struct Command {
    category: Category,
    command: Box<dyn Fn(&dyn SceneNode)>,
}

impl Command {
    pub fn new(category: Category, command: Box<dyn Fn(&dyn SceneNode)>) -> Command {
        Command { category, command }
    }

    pub fn get_category(&self) -> Category {
        self.category
    }

    pub fn execute(&self, scene_node: &dyn SceneNode) {
        (self.command)(scene_node);
    }
}
*/
