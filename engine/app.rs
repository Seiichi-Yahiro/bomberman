use crate::asset_storage::AssetStorage;
use crate::state_manager::{GameState, GameStateBuilder, StateManager};
use crate::traits::game_loop_event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
    window: Window,
    events: Events,
    opengl_version: OpenGL,
    asset_storage: AssetStorage,
}

impl App {
    pub fn new(
        window_settings: WindowSettings,
        event_settings: EventSettings,
        opengl_version: OpenGL,
    ) -> App {
        App {
            window: window_settings
                .graphics_api(opengl_version)
                .build()
                .unwrap_or_else(|e| panic!("Failed to build Window: {}", e)),
            events: Events::new(event_settings),
            opengl_version,
            asset_storage: AssetStorage::new(),
        }
    }

    pub fn run(&mut self, builder: GameStateBuilder) {
        let mut gl = GlGraphics::new(self.opengl_version);
        let mut state_manager = StateManager::new(builder, &mut self.asset_storage);

        while let (Some(event), false) =
            (self.events.next(&mut self.window), state_manager.is_empty())
        {
            state_manager.handle_event(&mut self.asset_storage, &event);

            if let Some(update_args) = event.update_args() {
                state_manager.update(&mut self.asset_storage, update_args.dt);
            }

            if let Some(render_args) = event.render_args() {
                gl.draw(render_args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    state_manager.draw(&c, g);
                });
            }
        }
    }
}
