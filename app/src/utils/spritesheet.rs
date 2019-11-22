use crate::utils::{load_tileset_textures, TextureMap};
use opengl_graphics::Texture;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Spritesheet {
    tileset: tiled::Tileset,
    textures: TextureMap,
    texture_names_to_ids: HashMap<String, u32>,
    current_texture: u32,
}

impl Spritesheet {
    pub fn new(folder: &str, tileset_file: &str, default_texture: &str) -> Spritesheet {
        let path = format!("{}{}", folder, tileset_file);
        let tileset = tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap();
        let texture_names_to_ids = Self::map_texture_names_to_ids(&tileset);

        Spritesheet {
            current_texture: texture_names_to_ids[default_texture],
            textures: load_tileset_textures(&tileset, folder),
            texture_names_to_ids,
            tileset,
        }
    }

    pub fn get_current_texture(&self) -> Rc<Texture> {
        Rc::clone(&self.textures[&self.current_texture])
    }

    pub fn set_current_texture(&mut self, texture_name: &str) {
        self.current_texture = self.texture_names_to_ids[texture_name];
    }

    fn map_texture_names_to_ids(tileset: &tiled::Tileset) -> HashMap<String, u32> {
        tileset
            .tiles
            .iter()
            .filter_map(|tile| {
                if let Some(tiled::PropertyValue::StringValue(texture_name)) =
                    tile.properties.get("name")
                {
                    Some((texture_name.clone(), tile.id))
                } else {
                    None
                }
            })
            .collect()
    }
}
