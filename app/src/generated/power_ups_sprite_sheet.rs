use graphics::types::SourceRectangle;
use opengl_graphics::{Texture, TextureSettings};

pub struct PowerUpsSpriteSheet {
    pub texture: Texture,
    pub fire_up: SourceRectangle,
    pub speed_up: SourceRectangle,
    pub life_up: SourceRectangle,
    pub bomb_down: SourceRectangle,
    pub bomb_kick: SourceRectangle,
    pub bomb_up: SourceRectangle,
    pub fire_down: SourceRectangle,
    pub speed_down: SourceRectangle,
}

impl PowerUpsSpriteSheet {
    pub fn new() -> PowerUpsSpriteSheet {
        PowerUpsSpriteSheet {
            texture: Texture::from_path(
                "app/assets/textures/power_ups_sprite_sheet.png",
                &TextureSettings::new(),
            )
            .unwrap(),
            fire_up: [0.0, 64.0, 32.0, 32.0],
            speed_up: [32.0, 32.0, 32.0, 32.0],
            life_up: [0.0, 96.0, 32.0, 32.0],
            bomb_down: [32.0, 96.0, 32.0, 32.0],
            bomb_kick: [0.0, 0.0, 32.0, 32.0],
            bomb_up: [32.0, 0.0, 32.0, 32.0],
            fire_down: [32.0, 64.0, 32.0, 32.0],
            speed_down: [0.0, 32.0, 32.0, 32.0],
        }
    }
}
