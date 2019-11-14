use crate::arena::Arena;
use crate::traits::controller::*;

pub struct ArenaController {
    pub arena: Arena,
}

impl Controller for ArenaController {
    fn event<E: GenericEvent>(&mut self, event: &E) {}
}
