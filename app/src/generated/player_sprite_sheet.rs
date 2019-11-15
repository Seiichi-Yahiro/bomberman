use graphics::types::SourceRectangle;
use opengl_graphics::{Texture, TextureSettings};

pub struct PlayerSpriteSheet {
    pub texture: Texture,
    pub bomber_down_standing: SourceRectangle,
    pub bomber_right_left: SourceRectangle,
    pub bomber_left_left: SourceRectangle,
    pub bomber_down_right: SourceRectangle,
    pub bomber_down_left: SourceRectangle,
    pub bomber_up_standing: SourceRectangle,
    pub bomber_left_standing: SourceRectangle,
    pub bomber_right_standing: SourceRectangle,
    pub bomber_up_left: SourceRectangle,
    pub bomber_up_right: SourceRectangle,
    pub bomber_right_right: SourceRectangle,
    pub bomber_left_right: SourceRectangle,
}

impl PlayerSpriteSheet {
    pub fn new() -> PlayerSpriteSheet {
        PlayerSpriteSheet {
            texture: Texture::from_path(
                "app/assets/textures/player_sprite_sheet.png",
                &TextureSettings::new(),
            )
            .unwrap(),
            bomber_down_standing: [64.0, 64.0, 32.0, 32.0],
            bomber_right_left: [0.0, 96.0, 32.0, 32.0],
            bomber_left_left: [0.0, 32.0, 32.0, 32.0],
            bomber_down_right: [32.0, 96.0, 32.0, 32.0],
            bomber_down_left: [64.0, 0.0, 32.0, 32.0],
            bomber_up_standing: [64.0, 32.0, 32.0, 32.0],
            bomber_left_standing: [0.0, 64.0, 32.0, 32.0],
            bomber_right_standing: [32.0, 32.0, 32.0, 32.0],
            bomber_up_left: [32.0, 64.0, 32.0, 32.0],
            bomber_up_right: [32.0, 0.0, 32.0, 32.0],
            bomber_right_right: [64.0, 96.0, 32.0, 32.0],
            bomber_left_right: [0.0, 0.0, 32.0, 32.0],
        }
    }
}
