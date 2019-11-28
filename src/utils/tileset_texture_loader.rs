use graphics::types::SourceRectangle;
use graphics::ImageSize;
use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct SpritesheetTextureHolder {
    texture_map: HashMap<u32, Rc<Texture>>,
    spritesheet_list: Vec<Spritesheet>,
}

impl SpritesheetTextureHolder {
    pub fn get_texture_data(&self, tile_id: u32) -> Option<TextureData> {
        if let Some(texture) = self.texture_map.get(&tile_id) {
            Some(TextureData::new(
                Rc::clone(texture),
                [
                    0.0,
                    0.0,
                    texture.get_width() as f64,
                    texture.get_height() as f64,
                ],
            ))
        } else if let Some(spritesheet) = self
            .spritesheet_list
            .iter()
            .find(|spritesheet| (spritesheet.contains)(tile_id))
        {
            Some(TextureData::new(
                Rc::clone(&spritesheet.texture),
                (spritesheet.create_src_rect)(tile_id),
            ))
        } else {
            None
        }
    }
}

struct Spritesheet {
    pub texture: Rc<Texture>,
    pub contains: Box<dyn Fn(u32) -> bool>,
    pub create_src_rect: Box<dyn Fn(u32) -> SourceRectangle>,
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

pub fn load_tileset(folder: &str, tileset_file: &str) -> tiled::Tileset {
    let path = format!("{}{}", folder, tileset_file);
    tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap()
}

pub fn load_tileset_textures_from_map(map: &tiled::Map, folder: &str) -> SpritesheetTextureHolder {
    map.tilesets
        .iter()
        .map(|tileset| load_tileset_textures(tileset, folder))
        .fold(SpritesheetTextureHolder::default(), |mut acc, item| {
            acc.spritesheet_list.extend(item.spritesheet_list);
            acc.texture_map.extend(item.texture_map);
            acc
        })
}

pub fn load_tileset_textures(tileset: &tiled::Tileset, folder: &str) -> SpritesheetTextureHolder {
    if let Some(tiled::Image {
        source,
        width,
        height,
        ..
    }) = tileset.images.first()
    {
        let tiled::Tileset {
            tile_width,
            tile_height,
            first_gid,
            ..
        } = tileset;
        let texture = load_texture(folder, source);
        let spritesheet_contains_tile_id = create_spritesheet_contains_tile_id_fn(
            *tile_width,
            *tile_height,
            *width as u32,
            *height as u32,
            *first_gid,
        );
        let create_src_rect =
            create_src_rect_fn(*tile_width, *tile_height, *width as u32, *first_gid);

        SpritesheetTextureHolder {
            texture_map: HashMap::new(),
            spritesheet_list: vec![Spritesheet {
                texture: Rc::new(texture),
                contains: spritesheet_contains_tile_id,
                create_src_rect,
            }],
        }
    } else {
        let texture_map: HashMap<u32, Rc<Texture>> = tileset
            .tiles
            .iter()
            .map(|tile| {
                let texture = load_texture(folder, &tile.images.first().unwrap().source);
                (tile.id + tileset.first_gid, Rc::new(texture))
            })
            .collect();

        SpritesheetTextureHolder {
            spritesheet_list: vec![],
            texture_map,
        }
    }
}

fn load_texture(folder: &str, source: &str) -> Texture {
    let path = format!("{}{}", folder, source);
    let texture_settings = TextureSettings::new();
    Texture::from_path(path, &texture_settings).unwrap()
}

fn create_spritesheet_contains_tile_id_fn(
    tile_width: u32,
    tile_height: u32,
    image_width: u32,
    image_height: u32,
    first_gid: u32,
) -> Box<dyn Fn(u32) -> bool> {
    let x_tiles = image_width / tile_width;
    let y_tiles = image_height / tile_height;
    let number_of_tiles = x_tiles * y_tiles;
    let last_gid = first_gid + number_of_tiles - 1;

    let spritesheet_contains_tile_id =
        move |tile_id: u32| tile_id >= first_gid && tile_id <= last_gid;

    Box::new(spritesheet_contains_tile_id)
}

fn create_src_rect_fn(
    tile_width: u32,
    tile_height: u32,
    image_width: u32,
    first_gid: u32,
) -> Box<dyn Fn(u32) -> SourceRectangle> {
    let columns = image_width / tile_width;
    let create_src_rect = move |tile_id: u32| -> SourceRectangle {
        let tile_id = tile_id - first_gid;
        let x = (tile_id % columns) * tile_width;
        let y = (tile_id / columns) * tile_height;
        [x as f64, y as f64, tile_width as f64, tile_height as f64]
    };

    Box::new(create_src_rect)
}
