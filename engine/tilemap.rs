use crate::asset_storage::Asset;
use crate::tileset::{TileId, TilePosition, Tileset};
use crate::utils::flatten_2d;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

pub struct Tilemap {
    pub object_groups: HashMap<String, Vec<tiled::Object>>,
    pub tiles: Vec<HashMap<TilePosition, TileId>>,
    pub tile_ids: Vec<TileId>,
    pub tileset: Rc<Tileset>,
    pub width: u32,
    pub height: u32,
}

impl Tilemap {
    fn convert_tilemap_to_tiles(tilemap: &tiled::Map) -> Vec<HashMap<TilePosition, TileId>> {
        let convert_layer_to_tiles = |layer: &tiled::Layer| {
            flatten_2d(&layer.tiles)
                .into_iter()
                .filter_map(|(row, column, &tile_id)| match tile_id {
                    0 => None,
                    _ => Some((
                        [
                            column as u32 * tilemap.tile_width,
                            row as u32 * tilemap.tile_height,
                        ],
                        tile_id,
                    )),
                })
                .collect()
        };

        tilemap.layers.iter().map(convert_layer_to_tiles).collect()
    }

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
            tiles: Self::convert_tilemap_to_tiles(&tilemap),
            object_groups: Self::extract_object_groups_from_tilemap(&tilemap),
            tileset: Rc::new(tileset),
        }
    }
}
