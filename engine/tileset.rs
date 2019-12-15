use crate::animation::{Animation, Frame};
use crate::asset_storage::Asset;
use crate::texture_holder::TextureHolder;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;

pub type TilePosition = [u32; 2];
pub type TileId = u32;
pub type TilesetId = Uuid;

#[derive(Default)]
pub struct Tileset {
    pub texture_holder: TextureHolder,
    pub animation_frames_holder: HashMap<TileId, Rc<Vec<Frame>>>,
}

impl Tileset {
    pub fn from_tileset(tileset: &tiled::Tileset, folder: &Path) -> Tileset {
        Tileset {
            texture_holder: TextureHolder::from_tileset(&tileset, folder),
            animation_frames_holder: Animation::load_animation_frames_from_tileset(&tileset),
        }
    }

    pub fn combine(&mut self, tileset: Tileset) {
        self.texture_holder.combine(tileset.texture_holder);
        self.animation_frames_holder
            .extend(tileset.animation_frames_holder);
    }
}

impl Asset for Tileset {
    fn load_from_file(path: &Path) -> Self
    where
        Self: Sized,
    {
        if !path.is_file() || !path.ends_with(".xml") {
            panic!(format!("{} is not a .xml file!", path.display()));
        }

        let tileset = tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap();

        Self::from_tileset(
            &tileset,
            &path.parent().unwrap_or_else(|| {
                panic!(format!(
                    "Cannot find parent directory of {}",
                    path.display()
                ))
            }),
        )
    }
}
