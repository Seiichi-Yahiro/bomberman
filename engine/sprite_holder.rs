use crate::animation::Animation;
use crate::texture_holder::SpriteTextureDataExt;
use crate::tileset::{TilePosition, Tileset};
use crate::traits::game_loop_event::{Drawable, Updatable};
use graphics::math::Matrix2d;
use graphics::ImageSize;
use opengl_graphics::{GlGraphics, Texture};
use sprite::Sprite as PistonSprite;
use std::rc::Rc;

pub struct SpriteHolder {
    pub sprite: PistonSprite<Texture>,
    pub animation: Option<Animation>,
    tileset: Rc<Tileset>,
    default_tile_id: u32,
}

impl SpriteHolder {
    pub fn from_tileset(tileset: Rc<Tileset>, tile_id: u32) -> Option<SpriteHolder> {
        let sprite =
            PistonSprite::from_texture_data(tileset.texture_holder.get_texture_data(tile_id)?);

        Some(SpriteHolder {
            sprite,
            animation: Self::get_animation(&tileset, tile_id),
            default_tile_id: tile_id,
            tileset,
        })
    }

    pub fn update_tile_id(&mut self, tile_id: u32) {
        if let Some(texture_data) = self.tileset.texture_holder.get_texture_data(tile_id) {
            if self.default_tile_id != tile_id {
                self.default_tile_id = tile_id;
                self.sprite.update_texture_data(texture_data);
                self.update_animation(tile_id);
            }
        }
    }

    fn update_animation(&mut self, tile_id: u32) {
        let mut new_animation = Self::get_animation(&self.tileset, tile_id);

        if let Some(new_animation) = new_animation.as_mut() {
            let was_playing = self
                .animation
                .as_ref()
                .map(|prev_animation| !prev_animation.is_stopped() && !prev_animation.is_paused())
                .unwrap_or(false);

            if was_playing {
                new_animation.play();
            }
        }

        self.animation = new_animation;
    }

    fn get_animation(tileset: &Rc<Tileset>, tile_id: u32) -> Option<Animation> {
        let frames = tileset.animation_frames_holder.get(&tile_id)?;
        Some(Animation::new(Rc::clone(frames)))
    }
}

impl Updatable for SpriteHolder {
    fn update(&mut self, dt: f64) {
        let default_tile_id = self.default_tile_id;
        let tile_id = self
            .animation
            .as_mut()
            .map_or(default_tile_id, |animation| {
                if !animation.is_stopped() {
                    animation.update(dt);
                    animation.get_current_tile_id()
                } else {
                    default_tile_id
                }
            });

        if let Some(texture_data) = self.tileset.texture_holder.get_texture_data(tile_id) {
            self.sprite.update_texture_data(texture_data)
        }
    }
}

impl Drawable for SpriteHolder {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.sprite.draw(transform, g);
    }
}
