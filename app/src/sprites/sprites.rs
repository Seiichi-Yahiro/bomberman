use graphics::types::SourceRectangle;
use opengl_graphics::{Texture, TextureSettings};
use serde_json::from_reader;
//use sprite::{Animation, Sprite};
use spritesheet_generator::spritesheet::{Frame, Spritesheet};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub fn load_spritesheet(
    texture_path: &Path,
    spritesheet_path: &Path,
) -> (Texture, HashMap<String, SourceRectangle>) {
    let file = File::open(spritesheet_path).unwrap_or_else(|e| {
        panic!(
            "Failed to open spritesheet json file ({}): {}",
            spritesheet_path.display(),
            e
        )
    });

    let spritesheet: Spritesheet = from_reader(file).unwrap_or_else(|e| {
        panic!(
            "Failed to parse spritesheet json({}): {}",
            spritesheet_path.display(),
            e
        )
    });

    let spritesheet = spritesheet
        .frames
        .into_iter()
        .map(
            |(
                name,
                Frame {
                    x,
                    y,
                    w,
                    h,
                    screen: _screen,
                },
            )| { (name, [x as f64, y as f64, w as f64, h as f64]) },
        )
        .collect();

    let texture = Texture::from_path(texture_path, &TextureSettings::new())
        .unwrap_or_else(|e| panic!("Failed to load texture ({}): {}", texture_path.display(), e));

    (texture, spritesheet)
}
