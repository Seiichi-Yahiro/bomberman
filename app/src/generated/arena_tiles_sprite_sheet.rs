use graphics::types::SourceRectangle;
use opengl_graphics::{Texture, TextureSettings};

pub struct ArenaTilesSpriteSheet {
    pub texture: Texture,
    pub soft_block: SourceRectangle,
    pub hard_block: SourceRectangle,
}

impl ArenaTilesSpriteSheet {
    pub fn new() -> ArenaTilesSpriteSheet {
        ArenaTilesSpriteSheet {
            texture: Texture::from_path(
                "app/assets/textures/arena_tiles_sprite_sheet.png",
                &TextureSettings::new(),
            )
            .unwrap(),
            soft_block: [32.0, 0.0, 32.0, 32.0],
            hard_block: [0.0, 0.0, 32.0, 32.0],
        }
    }
}
