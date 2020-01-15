use crate::tiles::tileset::Tileset;
use crate::utils::asset_storage::Asset;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

pub struct Tilemap {
    pub object_groups: HashMap<String, Vec<tiled::Object>>,
    pub tileset: Arc<Tileset>,
    pub width: u32,
    pub height: u32,
}

impl Tilemap {
    fn extract_object_groups_from_tilemap(
        tilemap: &tiled::Map,
    ) -> HashMap<String, Vec<tiled::Object>> {
        tilemap
            .object_groups
            .iter()
            .map(|group| {
                (
                    group.name.clone(),
                    group
                        .objects
                        .clone()
                        .into_iter()
                        .map(|mut object| {
                            object.y -= tilemap.tile_height as f32; // Objects origin is at bottom left
                            object
                        })
                        .collect(),
                )
            })
            .collect()
    }
}

impl Asset for Tilemap {
    fn load_from_file(path: &Path) -> Self
    where
        Self: Sized,
    {
        let path = Path::new(path);
        let is_tmx = path
            .extension()
            .and_then(OsStr::to_str)
            .map_or(false, |ext| ext == "tmx");

        if !path.is_file() || !is_tmx {
            panic!(format!("{} is not a .tmx file!", path.display()));
        }

        let tilemap = tiled::parse_file(path).unwrap();

        let tileset = tilemap
            .tilesets
            .iter()
            .map(|tileset| {
                Tileset::from_tileset(
                    tileset,
                    &path.parent().unwrap_or_else(|| {
                        panic!(format!(
                            "Cannot find parent directory of {}",
                            path.display()
                        ))
                    }),
                )
            })
            .fold(Tileset::default(), |mut acc, item| {
                acc.combine(item);
                acc
            });

        Tilemap {
            width: tilemap.width,
            height: tilemap.height,
            object_groups: Self::extract_object_groups_from_tilemap(&tilemap),
            tileset: Arc::new(tileset),
        }
    }
}
