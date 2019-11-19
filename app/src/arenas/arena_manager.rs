use crate::arenas::Arena;
use crate::generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet;
use crate::traits::game_loop_event::*;
use graphics::Transformed;
use sprite::Sprite;
use std::rc::Rc;

pub struct ArenaManager {
    pub arena: Arena,
    pub spritesheet: ArenaTilesSpriteSheet,
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        for y in 0..self.arena.height as usize {
            for x in 0..self.arena.width as usize {
                let mut sprite = {
                    let tile = &self.arena.tiles[y * self.arena.width as usize + x];
                    let rect = *self.spritesheet.get(tile.get_value());
                    let texture = Rc::clone(&self.spritesheet.texture);
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
