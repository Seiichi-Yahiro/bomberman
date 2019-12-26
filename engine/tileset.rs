use crate::animation::{Animation, Frame};
use crate::asset_storage::Asset;
use crate::texture_holder::TextureHolder;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

pub type TilePosition = [u32; 2];
pub type TileId = u32;

#[derive(Default)]
pub struct Tileset {
    pub texture_holder: TextureHolder,
    pub animation_frames_holder: HashMap<TileId, Arc<Vec<Frame>>>,
    pub properties: HashMap<TileId, tiled::Properties>,
}

impl Tileset {
    pub fn from_tileset(tileset: &tiled::Tileset, folder: &Path, from_tilemap: bool) -> Tileset {
        Tileset {
            texture_holder: TextureHolder::from_tileset(&tileset, folder),
            animation_frames_holder: Animation::load_animation_frames_from_tileset(
                &tileset,
                from_tilemap,
            ),
            properties: tileset
                .tiles
                .iter()
                .map(|tile| {
                    (
                        if !from_tilemap {
                            tile.id + tileset.first_gid
                        } else {
                            tile.id
                        },
                        tile.properties.clone(),
                    )
                })
                .collect(),
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
        let is_xml = path
            .extension()
            .and_then(OsStr::to_str)
            .map_or(false, |ext| ext == "xml");

        if !path.is_file() || !is_xml {
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
            false,
        )
    }
}
