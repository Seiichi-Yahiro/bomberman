use crate::enums::Tile;
use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::DenseVecStorage,
    error::Error,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Arena {
    pub tiles: Vec<Vec<Tile>>,
}

impl Asset for Arena {
    const NAME: &'static str = "arena::Arena";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

impl From<Arena> for Result<ProcessingState<Arena>, Error> {
    fn from(arena: Arena) -> Self {
        Ok(ProcessingState::Loaded(arena))
    }
}
