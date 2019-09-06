use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::DenseVecStorage,
    error::Error,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ArenaData {
    pub name: String,
    pub file: String,
}

#[derive(Deserialize, Debug)]
pub struct Arenas {
    pub arenas: Vec<ArenaData>,
}

impl Asset for Arenas {
    const NAME: &'static str = "asset::Arenas";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

impl From<Arenas> for Result<ProcessingState<Arenas>, Error> {
    fn from(arenas: Arenas) -> Self {
        Ok(ProcessingState::Loaded(arenas))
    }
}
