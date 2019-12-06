use graphics::types::SourceRectangle;
use graphics::ImageSize;
use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct TextureHolder {
    texture_map: TextureMap,
    spritesheet_list: Vec<Spritesheet>,
}

impl TextureHolder {
    pub fn from_map(map: &tiled::Map, folder: &str) -> TextureHolder {
        map.tilesets
            .iter()
            .map(|tileset| Self::from_tileset(tileset, folder))
            .fold(TextureHolder::default(), |mut acc, item| {
                acc.combine(item);
                acc
            })
    }

    pub fn from_tileset(tileset: &tiled::Tileset, folder: &str) -> TextureHolder {
        if let Some(image) = tileset.images.first() {
            TextureHolder {
                texture_map: TextureMap::default(),
                spritesheet_list: vec![Spritesheet {
                    texture: Rc::new(Self::load_texture(folder, &image.source)),
                    tile_width: tileset.tile_width,
                    tile_height: tileset.tile_height,
                    first_gid: tileset.first_gid,
                }],
            }
        } else {
            let texture_map = tileset
                .tiles
                .iter()
                .map(|tile| {
                    let texture = Self::load_texture(folder, &tile.images.first().unwrap().source);
                    (tile.id + tileset.first_gid, Rc::new(texture))
                })
                .collect();

            TextureHolder {
                spritesheet_list: vec![],
                texture_map: TextureMap::new(texture_map),
            }
        }
    }

    fn load_texture(folder: &str, source: &str) -> Texture {
        let path = format!("{}{}", folder, source);
        let texture_settings = TextureSettings::new();
        Texture::from_path(path, &texture_settings).unwrap()
    }

    pub fn combine(&mut self, texture_holder: TextureHolder) {
        self.texture_map.map.extend(texture_holder.texture_map.map);
        self.spritesheet_list
            .extend(texture_holder.spritesheet_list);
    }

    pub fn get_texture_data(&self, tile_id: u32) -> Option<TextureData> {
        self.spritesheet_list
            .iter()
            .find_map(|spritesheet| spritesheet.get_texture_data(tile_id))
            .or_else(|| self.texture_map.get_texture_data(tile_id))
    }
}

struct TextureMap {
    pub map: HashMap<u32, Rc<Texture>>,
}

impl Default for TextureMap {
    fn default() -> Self {
        TextureMap {
            map: HashMap::new(),
        }
    }
}

impl TextureMap {
    pub fn new(map: HashMap<u32, Rc<Texture>>) -> TextureMap {
        TextureMap { map }
    }

    pub fn get_texture_data(&self, tile_id: u32) -> Option<TextureData> {
        self.map.get(&tile_id).map(|texture| {
            TextureData::new(
                Rc::clone(texture),
                [
                    0.0,
                    0.0,
                    texture.get_width() as f64,
                    texture.get_height() as f64,
                ],
            )
        })
    }
}

struct Spritesheet {
    pub texture: Rc<Texture>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub first_gid: u32,
}

impl Spritesheet {
    pub fn contains(&self, tile_id: u32) -> bool {
        let x_tiles = self.texture.get_width() / self.tile_width;
        let y_tiles = self.texture.get_height() / self.tile_height;
        let number_of_tiles = x_tiles * y_tiles;
        let last_gid = self.first_gid + number_of_tiles;

        (self.first_gid..last_gid).contains(&tile_id)
    }

    pub fn get_src_rect(&self, tile_id: u32) -> Option<SourceRectangle> {
        if self.contains(tile_id) {
            let columns = self.texture.get_width() / self.tile_width;
            let tile_id = tile_id - self.first_gid;
            let x = (tile_id % columns) * self.tile_width;
            let y = (tile_id / columns) * self.tile_height;
            Some([
                x as f64,
                y as f64,
                self.tile_width as f64,
                self.tile_height as f64,
            ])
        } else {
            None
        }
    }

    pub fn get_texture_data(&self, tile_id: u32) -> Option<TextureData> {
        self.get_src_rect(tile_id)
            .map(|rect| TextureData::new(Rc::clone(&self.texture), rect))
    }
}

#[derive(Clone)]
pub struct TextureData {
    pub texture: Rc<Texture>,
    pub src_rect: SourceRectangle,
}

impl TextureData {
    pub fn new(texture: Rc<Texture>, src_rect: SourceRectangle) -> TextureData {
        TextureData { texture, src_rect }
    }
}
