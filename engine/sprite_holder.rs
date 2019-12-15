use crate::animation::{Animation, Frame};
use crate::asset_storage::AssetStorage;
use crate::texture_holder::{SpriteTextureDataExt, TextureData};
use crate::tileset::Tileset;
use crate::traits::game_loop_event::{Drawable, Updatable};
use crate::world::World;
use graphics::Context;
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
    /*    pub fn new(
        tile_id: u32,
        texture_data: TextureData,
        animation_frames: Option<Rc<Vec<Frame>>>,
    ) -> SpriteHolder {
        SpriteHolder {
            sprite: PistonSprite::from_texture_data(texture_data),
            animation: animation_frames.map(|frames| Animation::new(frames)),
            default_tile_id: tile_id,
        }
    }*/

    pub fn from_tileset(tileset: Rc<Tileset>, tile_id: u32) -> Option<SpriteHolder> {
        Some(SpriteHolder {
            sprite: PistonSprite::from_texture_data(
                tileset.texture_holder.get_texture_data(tile_id)?,
            ),
            animation: tileset
                .animation_frames_holder
                .get(&tile_id)
                .map(|frames| Animation::new(Rc::clone(frames))),
            default_tile_id: tile_id,
            tileset,
        })
    }
}

impl Updatable for SpriteHolder {
    fn update(&mut self, _world: &mut World, dt: f64) {
        let default_tile_id = self.default_tile_id;
        let tile_id = self
            .animation
            .as_mut()
            .map_or(default_tile_id, |animation| {
                if animation.is_playing() {
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
    fn draw(&self, _world: &World, c: &Context, g: &mut GlGraphics) {
        self.sprite.draw(c.transform, g);
    }
}
