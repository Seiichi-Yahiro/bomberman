use crate::arena::Arena;
use crate::generated::arena_tiles_sprite_sheet::ArenaTilesSpriteSheet;
use crate::traits::controller::*;

pub struct ArenaController {
    pub arena: Arena,
    pub spritesheet: ArenaTilesSpriteSheet,
}

impl Controller for ArenaController {
    fn event<E: GenericEvent>(&mut self, event: &E) {}
}
