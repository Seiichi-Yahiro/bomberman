extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate inflections;
extern crate regex;
extern crate serde;
extern crate texture_packer;

use image::Rgba;
use inflections::Inflect;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::{
    fs::{read_dir, File},
    path::PathBuf,
};
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, texture::Texture, Frame, TexturePacker,
    TexturePackerConfig,
};

const ASSETS: &str = "codegen/assets/textures/";
const OUTPUT_ASSETS: &str = "app/assets/textures/generated/";
const OUTPUT_RUST: &str = "app/src/generated/";
const SPRITE_SHEET: &str = "_sprite_sheet";

fn main() {
    let mut mod_rs = File::create(OUTPUT_RUST.to_string() + "mod.rs").unwrap();
    [
        (32 * 3, 32 * 4 + 1, "player"),
        (32 * 4, 32 * 1 + 1, "bomb"),
        (32 * 3, 32 * 1 + 1, "arena_tiles"),
        (32 * 2, 32 * 4 + 1, "power_ups"),
    ]
    .iter()
    .for_each(|(width, height, name)| {
        mod_rs
            .write(format!("pub mod {}{};", name, SPRITE_SHEET).as_bytes())
            .unwrap();
        create_spritesheet_files(*width, *height, name);
    });
}

fn create_spritesheet_files(width: u32, height: u32, filename: &str) {
    let input_folder = ASSETS.to_string() + filename;
    let packer = {
        let config = TexturePackerConfig {
            max_width: width,
            max_height: height,
            border_padding: 0,
            texture_padding: 0,
            trim: false,
            allow_rotation: false,
            texture_outlines: false,
        };

        let mut packer = TexturePacker::new_skyline(config);
        for file_textures in find_all_files(input_folder) {
            packer.pack_own(file_textures.file.name, file_textures.texture);
        }

        packer
    };

    let asset_output_folder = OUTPUT_ASSETS.to_string() + filename + SPRITE_SHEET + ".png";
    create_spritesheet(&asset_output_folder, &packer);
    create_struct(
        packer.get_frames(),
        &(filename.to_string() + SPRITE_SHEET),
        &asset_output_folder,
    );
}

fn create_spritesheet<T: Texture<Pixel = Rgba<u8>>>(folder: &str, texture: &T) {
    let mut image_file = File::create(folder).unwrap();
    ImageExporter::export(texture)
        .unwrap()
        .write_to(&mut image_file, image::PNG)
        .unwrap();
}

fn create_struct(frames: &HashMap<String, Frame>, filename: &str, asset_output_folder: &str) {
    let rust = {
        let struct_name = filename[0..1].to_uppercase() + &filename[1..].to_camel_case();
        let mut struct_def = String::from("pub texture: Rc<Texture>,");
        let mut struct_init = format!(
            "texture: Rc::new(Texture::from_path(\"{}\", &TextureSettings::new()).unwrap()),",
            asset_output_folder
        );
        let mut rects_insert = String::new();

        for (name, frame) in frames {
            struct_def.push_str(&format!("pub {}: Rc<SourceRectangle>,", name));

            let value = format!(
                "[{}.0,{}.0,{}.0,{}.0]",
                frame.frame.x, frame.frame.y, frame.frame.w, frame.frame.h
            );
            rects_insert.push_str(&format!("map.insert(\"{}\", Rc::new({}));", name, value));
            struct_init.push_str(&format!(
                "{name}: rects.get(\"{name}\").unwrap().clone(),",
                name = name
            ));
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        format!(
            "use graphics::types::SourceRectangle;
            use opengl_graphics::{{Texture, TextureSettings}};
            use std::rc::Rc;
            use std::collections::HashMap;

            pub struct {struct_name} {{
                {struct_def}
                rects: HashMap<&'static str, Rc<SourceRectangle>>,
            }}

            impl {struct_name} {{
                pub fn new() -> {struct_name} {{
                    let rects = {{
                        let mut map = HashMap::<&'static str, Rc<SourceRectangle>>::new();
                        {rects_insert}
                        map
                    }};

                    {struct_name} {{
                        {struct_init}
                        rects,
                    }}
                }}

                pub fn get(&self, key: &str) -> &SourceRectangle {{
                    self.rects.get(key).unwrap()
                }}
            }}",
            struct_name=struct_name, struct_def=struct_def, struct_init=struct_init, rects_insert=rects_insert
        )
    };

    File::create(OUTPUT_RUST.to_string() + filename + ".rs")
        .unwrap()
        .write(rust.as_bytes())
        .unwrap();
}

// Below here copied from spritesheet-generator

fn find_all_files(folder: String) -> Vec<FileTexture> {
    let mut file_list = Vec::new();
    if let Ok(entries) = read_dir(&folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let file_name = entry.file_name().into_string().unwrap();
                        let child_folder = format!("{}{}/", &folder, file_name);
                        file_list.extend(find_all_files(child_folder));
                    } else {
                        let file = extract_file_data(entry.path());
                        let texture = ImageImporter::import_from_file(&entry.path()).unwrap();
                        file_list.push(FileTexture { file, texture });
                    }
                }
            }
        }
    }
    file_list
}

#[derive(Serialize, Deserialize)]
pub struct FileData {
    pub path: PathBuf,
    pub name: String,
    pub ext: String,
}

pub fn extract_file_data(path: PathBuf) -> FileData {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<name>[^:\\/]*?)(?:\.(?P<ext>[^ :\\/.]*))?$").unwrap();
    }
    let file = RE.captures(&path.to_str().unwrap()).unwrap();

    FileData {
        path: path.clone(),
        name: String::from(&file["name"]),
        ext: String::from(&file["ext"]),
    }
}

struct FileTexture {
    pub file: FileData,
    pub texture: image::DynamicImage,
}
