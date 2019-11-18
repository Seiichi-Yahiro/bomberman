pub mod controller {
    pub use piston::input::GenericEvent;

    pub trait Controller {
        fn event<E: GenericEvent>(&mut self, event: &E);
    }
}

pub mod view {
    pub use super::controller::*;
    pub use graphics::{Context, Graphics};
    pub use opengl_graphics::Texture;

    pub trait View {
        type RelatedController;

        fn draw<G>(&self, controller: &Self::RelatedController, c: &Context, g: &mut G)
        where
            Self::RelatedController: Controller,
            G: Graphics<Texture = Texture>;
    }
}
