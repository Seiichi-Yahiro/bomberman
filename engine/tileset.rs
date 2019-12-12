use crate::animation::{load_animation_frames_from_tileset, Frame};
use crate::asset_storage::Asset;
use crate::texture_holder::TextureHolder;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Default)]
pub struct Tileset {
    pub texture_holder: TextureHolder,
    pub animation_frames_holder: HashMap<u32, Vec<Frame>>,
}

impl Tileset {
    pub fn from_tileset<E: Error>(tileset: &tiled::Tileset, folder: &Path) -> Result<Tileset, E> {
        let tileset = Tileset {
            texture_holder: TextureHolder::from_tileset(&tileset, folder)?,
            animation_frames_holder: load_animation_frames_from_tileset(&tileset),
        };

        Ok(tileset)
    }

    pub fn combine(&mut self, tileset: Tileset) {
        self.texture_holder.combine(tileset.texture_holder);
        self.animation_frames_holder
            .extend(tileset.animation_frames_holder);
    }
}

impl Asset for Tileset {
    fn load_from_file<E>(path: &str) -> Result<Self, E>
    where
        Self: Sized,
        E: Error,
    {
        if !path.is_file() || !path.ends_with(".xml") {
            return Err(format!("{} is not a .xml file!", path));
        }

        let tileset = tiled::parse_tileset(std::fs::File::open(path)?, 1)?;

        Self::from_tileset(
            &tileset,
            &path
                .parent()
                .ok_or(format!("Cannot find parent directory of {}", path))?,
        )
    }
}
