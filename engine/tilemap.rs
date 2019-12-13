use crate::asset_storage::Asset;
use crate::tileset::Tileset;
use crate::utils::flatten_2d;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub struct Tile {
    pub y: u32,
    pub x: u32,
    pub tile_id: u32,
}

pub struct Tilemap {
    pub tiles: Vec<Vec<Tile>>,
    pub tileset: Rc<Tileset>,
    pub object_groups: HashMap<String, Vec<tiled::Object>>,
}

impl Tilemap {
    fn convert_tilemap_to_tiles(tile_map: &tiled::Map) -> Vec<Vec<Tile>> {
        let convert_layer_to_tiles = |layer: &tiled::Layer| {
            flatten_2d(&layer.tiles)
                .into_iter()
                .map(|(row, column, &tile_id)| Tile {
                    y: row as u32 * tile_map.tile_height,
                    x: column as u32 * tile_map.tile_width,
                    tile_id,
                })
                .collect()
        };

        tile_map.layers.iter().map(convert_layer_to_tiles).collect()
    }

    fn extract_object_groups_from_tile_map(
        tile_map: &tiled::Map,
    ) -> HashMap<String, Vec<tiled::Object>> {
        tile_map
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
                            object.y -= tile_map.tile_height as f32; // Objects origin is at bottom left
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
        if !path.is_file() || !path.ends_with(".tmx") {
            panic!(format!("{} is not a .tmx file!", path.display()));
        }

        let tile_map = tiled::parse_file(path).unwrap();

        let tileset = tile_map
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
            tiles: Self::convert_tilemap_to_tiles(&tile_map),
            object_groups: Self::extract_object_groups_from_tile_map(&tile_map),
            tileset: Rc::new(tileset),
        }
    }
}
