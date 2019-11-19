use crate::traits::game_loop_event::*;
use graphics::Transformed;
use opengl_graphics::{Texture, TextureSettings};
use sprite::Sprite;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use tiled::parse_file;
use tiled::Map;

const ARENAS_FOLDER: &str = "app/assets/arenas/";
const FILE_NAME: &str = "arena_classic.tmx";

pub struct ArenaManager {
    pub arena: Map,
    pub textures: HashMap<u32, Rc<Texture>>,
}

impl ArenaManager {
    pub fn new() -> ArenaManager {
        let arena = {
            let path = format!("{}{}", ARENAS_FOLDER, FILE_NAME);
            parse_file(&Path::new(&path)).unwrap()
        };

        println!("{:?}", arena.object_groups[0].objects);

        let textures: HashMap<u32, Rc<Texture>> = arena.tilesets[0]
            .tiles
            .iter()
            .map(|tile| {
                let path = format!("{}{}", ARENAS_FOLDER, tile.images.first().unwrap().source);
                let texture_settings = &TextureSettings::new();
                let texture = Texture::from_path(path, &texture_settings).unwrap();
                (tile.id, Rc::new(texture))
            })
            .collect();

        ArenaManager { arena, textures }
    }
}

impl GameLoopEvent<()> for ArenaManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let layer = &self.arena.layers[0];

        let (tile_width, tile_height) = (self.arena.tile_width, self.arena.tile_height);

        for (y, row) in layer.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == 0 {
                    continue;
                }

                let transform = c
                    .transform
                    .trans(x as f64 * tile_width as f64, y as f64 * tile_height as f64);

                let texture = Rc::clone(&self.textures[&(tile - 1)]);
                let mut sprite = Sprite::from_texture(texture);
                sprite.set_anchor(0.0, 0.0);
                sprite.draw(transform, g);
            }
        }
    }
}
