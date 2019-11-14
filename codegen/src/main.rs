extern crate spritesheet_generator;

use spritesheet_generator::spritesheet_generator::generate;
use spritesheet_generator::spritesheet_generator_config::SpritesheetGeneratorConfig;

fn main() {
    let config = SpritesheetGeneratorConfig {
        max_width: 224,
        max_height: 128,
        border_padding: 0,
        input_folder: "codegen/assets/textures/".to_string(),
        output_folder: "app/assets/textures/".to_string(),
        output_file_name: "sprite_sheet".to_string(),
        allow_rotation: false,
    };
    generate(config);
}
