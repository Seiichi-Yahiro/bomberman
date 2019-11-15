use graphics::types::SourceRectangle;
use opengl_graphics::{Texture, TextureSettings};

pub struct BombSpriteSheet {
    pub texture: Texture,
    pub bomb: SourceRectangle,
    pub explosion_ray: SourceRectangle,
    pub explosion_middle: SourceRectangle,
    pub explosion_end: SourceRectangle,
}

impl BombSpriteSheet {
    pub fn new() -> BombSpriteSheet {
        BombSpriteSheet {
            texture: Texture::from_path(
                "app/assets/textures/bomb_sprite_sheet.png",
                &TextureSettings::new(),
            )
            .unwrap(),
            bomb: [32.0, 0.0, 32.0, 32.0],
            explosion_ray: [64.0, 0.0, 32.0, 32.0],
            explosion_middle: [96.0, 0.0, 32.0, 32.0],
            explosion_end: [0.0, 0.0, 32.0, 32.0],
        }
    }
}
