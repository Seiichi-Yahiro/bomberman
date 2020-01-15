use crate::tiles::animation::{Animation, Frame};
use crate::tiles::texture_holder::TextureHolder;
use crate::utils::asset_storage::Asset;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

pub type TileId = u32;
pub type HitBox = [f64; 4];

#[derive(Default)]
pub struct Tileset {
    pub texture_holder: TextureHolder,
    pub animation_frames_holder: HashMap<TileId, Arc<Vec<Frame>>>,
    pub properties: HashMap<TileId, tiled::Properties>,
    pub hit_boxes: HashMap<TileId, HitBox>,
}

impl Tileset {
    pub fn from_tileset(tileset: &tiled::Tileset, folder: &Path) -> Tileset {
        Tileset {
            texture_holder: TextureHolder::from_tileset(&tileset, folder),
            animation_frames_holder: Animation::load_animation_frames_from_tileset(&tileset),
            properties: Self::get_properties(tileset),
            hit_boxes: Self::get_hit_boxes(tileset),
        }
    }

    fn get_properties(tileset: &tiled::Tileset) -> HashMap<TileId, tiled::Properties> {
        tileset
            .tiles
            .iter()
            .map(|tile| (tile.id + tileset.first_gid, tile.properties.clone()))
            .collect()
    }

    fn get_hit_boxes(tileset: &tiled::Tileset) -> HashMap<TileId, HitBox> {
        tileset
            .tiles
            .iter()
            .filter_map(|tile| {
                let object = tile.objectgroup.as_ref()?.objects.first()?;
                match object.shape {
                    tiled::ObjectShape::Rect { width, height } => Some((
                        tile.id + tileset.first_gid,
                        [
                            object.x as f64,
                            object.y as f64,
                            width as f64,
                            height as f64,
                        ],
                    )),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn combine(&mut self, tileset: Tileset) {
        self.texture_holder.combine(tileset.texture_holder);
        self.animation_frames_holder
            .extend(tileset.animation_frames_holder);
        self.properties.extend(tileset.properties);
        self.hit_boxes.extend(tileset.hit_boxes);
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
        )
    }
}
