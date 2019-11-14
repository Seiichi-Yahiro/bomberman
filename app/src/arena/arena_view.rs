use crate::traits::view::*;

pub struct ArenaView {}

impl View for ArenaView {
    type RelatedController = crate::arena::ArenaController;

    fn draw<G>(&self, controller: &Self::RelatedController, c: &Context, g: &mut G)
    where
        Self::RelatedController: Controller,
        G: Graphics,
    {
    }
}
