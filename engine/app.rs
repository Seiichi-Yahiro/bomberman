use crate::asset_storage::{AssetStorage, AssetStorageResource};
use crate::game_state_builder::GameStateBuilder;
use crate::state_manager::StateManager;
use crate::traits::game_loop_event::*;
use glutin_window::GlutinWindow as Window;
use legion::world::Universe;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

pub struct AppData {
    pub asset_storage: AssetStorageResource,
    pub universe: Universe,
}

pub struct App {
    window: Window,
    events: Events,
    opengl_version: OpenGL,
    button_storage: HashSet<Button>,
    data: AppData,
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
            button_storage: HashSet::new(),
            data: AppData {
                asset_storage: Arc::new(RwLock::new(AssetStorage::new())),
                universe: Universe::new(),
            },
        }
    }

    pub fn run(&mut self, game_state_builder: GameStateBuilder) {
        let mut gl = GlGraphics::new(self.opengl_version);
        let mut state_manager = StateManager::new(game_state_builder, &mut self.data);

        while let (Some(event), false) =
            (self.events.next(&mut self.window), state_manager.is_empty())
        {
            if let Some(button_args) = event.button_args() {
                match button_args.state {
                    ButtonState::Press => {
                        // prevent repeated key pressed events when key is hold down
                        if !self.button_storage.contains(&button_args.button) {
                            self.button_storage.insert(button_args.button);
                            state_manager.handle_event(&mut self.data, &event);
                        }
                    }
                    ButtonState::Release => {
                        self.button_storage.remove(&button_args.button);
                        state_manager.handle_event(&mut self.data, &event);
                    }
                }
            }

            if let Some(update_args) = event.update_args() {
                state_manager.update(&mut self.data, update_args.dt);
            }

            if let Some(render_args) = event.render_args() {
                gl.draw(render_args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    state_manager.draw(&self.data, c.transform, g);
                });
            }
        }
    }
}
