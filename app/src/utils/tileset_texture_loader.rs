use graphics::types::SourceRectangle;
use graphics::ImageSize;
use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;
use std::rc::Rc;

pub enum SpritesheetTextureMap {
    Spritesheet(Rc<Texture>, Box<dyn Fn(u32) -> SourceRectangle>),
    SpriteCollection(HashMap<u32, Rc<Texture>>),
}

pub struct TextureData {
    pub texture: Rc<Texture>,
    pub src_rect: SourceRectangle,
}

impl TextureData {
    pub fn new(texture: Rc<Texture>, src_rect: SourceRectangle) -> TextureData {
        TextureData { texture, src_rect }
    }
}

impl SpritesheetTextureMap {
    pub fn get(&self, tile_id: u32) -> TextureData {
        match self {
            SpritesheetTextureMap::Spritesheet(texture, create_src_rect) => {
                TextureData::new(Rc::clone(texture), create_src_rect(tile_id))
            }
            SpritesheetTextureMap::SpriteCollection(texture_map) => {
                let texture = texture_map.get(&tile_id).unwrap_or_else(|| {
                    panic!(format!("Could not find texture with tile_id {}!", tile_id));
                });
                TextureData::new(
                    Rc::clone(texture),
                    [
                        0.0,
                        0.0,
                        texture.get_width() as f64,
                        texture.get_height() as f64,
                    ],
                )
            }
        }
    }
}

pub fn load_tileset(folder: &str, tileset_file: &str) -> tiled::Tileset {
    let path = format!("{}{}", folder, tileset_file);
    tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap()
}

pub fn load_tileset_textures(tileset: &tiled::Tileset, folder: &str) -> SpritesheetTextureMap {
    if !tileset.images.is_empty() {
        let tiled::Image { source, width, .. } = tileset.images.first().unwrap();

        let tile_width = tileset.tile_width;
        let tile_height = tileset.tile_height;
        let columns = *width as u32 / tileset.tile_width;

        let path = format!("{}{}", folder, source);
        let texture_settings = TextureSettings::new();
        let texture = Texture::from_path(path, &texture_settings).unwrap();

        let create_src_rect = move |tile_id: u32| -> SourceRectangle {
            let x = (tile_id % columns) * tile_width;
            let y = (tile_id / columns) * tile_height;
            [x as f64, y as f64, tile_width as f64, tile_height as f64]
        };

        SpritesheetTextureMap::Spritesheet(Rc::new(texture), Box::new(create_src_rect))
    } else {
        let texture_map: HashMap<u32, Rc<Texture>> = tileset
            .tiles
            .iter()
            .map(|tile| {
                let path = format!("{}{}", folder, tile.images.first().unwrap().source);
                let texture_settings = TextureSettings::new();
                let texture = Texture::from_path(path, &texture_settings).unwrap();
                (tile.id, Rc::new(texture))
            })
            .collect();

        SpritesheetTextureMap::SpriteCollection(texture_map)
    }
}
