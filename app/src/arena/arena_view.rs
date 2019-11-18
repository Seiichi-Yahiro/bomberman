use crate::traits::view::*;
use sprite::Sprite;
use std::rc::Rc;

pub struct ArenaView {}

impl View for ArenaView {
    type RelatedController = crate::arena::ArenaController;

    fn draw<G>(&self, controller: &Self::RelatedController, c: &Context, g: &mut G)
    where
        Self::RelatedController: Controller,
        G: Graphics<Texture = Texture>,
    {
        let mut sprite = Sprite::from_texture_rect(
            Rc::clone(&controller.spritesheet.texture),
            controller.spritesheet.hard_block,
        );
        sprite.set_anchor(0.0, 0.0);
        sprite.draw(c.transform, g);
    }
}
