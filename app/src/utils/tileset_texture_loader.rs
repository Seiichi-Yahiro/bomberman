use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;
use std::rc::Rc;

pub type TextureMap = HashMap<u32, Rc<Texture>>;

pub fn load_tileset_textures(tileset: &tiled::Tileset, folder: &str) -> TextureMap {
    tileset
        .tiles
        .iter()
        .map(|tile| {
            let path = format!("{}{}", folder, tile.images.first().unwrap().source);
            let texture_settings = TextureSettings::new();
            let texture = Texture::from_path(path, &texture_settings).unwrap();
            (tile.id, Rc::new(texture))
        })
        .collect()
}
