use crate::traits::view::*;
use graphics::Transformed;
use sprite::Sprite;
use std::rc::Rc;

pub struct ArenaView {}

impl ArenaView {
    pub fn new() -> ArenaView {
        ArenaView {}
    }
}

impl View for ArenaView {
    type RelatedController = crate::arenas::ArenaController;

    fn draw<G>(&self, controller: &Self::RelatedController, c: &Context, g: &mut G)
    where
        Self::RelatedController: Controller,
        G: Graphics<Texture = Texture>,
    {
        for y in 0..controller.arena.height as usize {
            for x in 0..controller.arena.width as usize {
                let mut sprite = {
                    let tile = &controller.arena.tiles[y * controller.arena.width as usize + x];
                    let rect = *controller.spritesheet.get(tile.get_value());
                    let texture = Rc::clone(&controller.spritesheet.texture);
                    Sprite::from_texture_rect(texture, rect)
                };

                sprite.set_anchor(0.0, 0.0);

                let [_xx, _yy, w, h] = sprite.get_src_rect().unwrap();
                let transform = c.transform.trans(x as f64 * w, y as f64 * h);

                sprite.draw(transform, g);
            }
        }
    }
}
