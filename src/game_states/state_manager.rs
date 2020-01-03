use crate::game_states::game_state_builder::GameStateBuilder;
use crate::utils::asset_storage::AssetStorage;
use glutin_window::OpenGL;
use graphics::Graphics;
use legion::world::Universe;
use opengl_graphics::GlGraphics;
use piston::input::{Button, ButtonEvent, ButtonState, Event, RenderEvent};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub trait GameState {
    fn execute(&mut self, event: Event) -> bool;
}

pub enum StateTransition {
    Push(GameStateBuilder),
    Pop,
    Switch(GameStateBuilder),
    Clear,
}

pub struct Resources {
    pub gl: Rc<RefCell<GlGraphics>>,
    pub universe: Arc<Universe>,
    pub pending_transitions: Arc<Mutex<VecDeque<StateTransition>>>,
    pub asset_storage: Arc<Mutex<AssetStorage>>,
    pub button_storage: Arc<Mutex<HashSet<Button>>>,
}

pub struct StateManager {
    stack: Vec<Box<dyn GameState>>,
    resources: Resources,
}

impl StateManager {
    pub fn new(game_state_builder: GameStateBuilder, opengl_version: OpenGL) -> StateManager {
        let mut state_manager = StateManager {
            stack: vec![],
            resources: Resources {
                gl: Rc::new(RefCell::new(GlGraphics::new(opengl_version))),
                universe: Arc::new(Universe::new()),
                pending_transitions: Arc::new(Mutex::new(VecDeque::from(vec![
                    StateTransition::Push(game_state_builder),
                ]))),
                asset_storage: Arc::new(Mutex::new(AssetStorage::new())),
                button_storage: Arc::new(Mutex::new(HashSet::new())),
            },
        };
        state_manager.apply_pending_transitions();
        state_manager
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn apply_pending_transitions(&mut self) {
        let mut pending_transitions = self.resources.pending_transitions.lock().unwrap();

        while let Some(transition) = pending_transitions.pop_front() {
            match transition {
                StateTransition::Push(builder) => {
                    self.stack.push((builder.build)(&self.resources));
                }
                StateTransition::Pop => {
                    self.stack.pop();
                }
                StateTransition::Switch(builder) => {
                    self.stack.pop();
                    self.stack.push((builder.build)(&self.resources));
                }
                StateTransition::Clear => self.stack.clear(),
            }
        }
    }

    fn update(&mut self, event: Event) {
        for state in self.stack.iter_mut().rev() {
            // if should not pass down
            if !state.execute(event.clone()) {
                break;
            }
        }

        self.apply_pending_transitions();
    }

    fn draw(&mut self, event: Event) {
        self.resources.gl.borrow_mut().clear_color([1.0; 4]);
        self.stack.iter_mut().for_each(|state| {
            state.execute(event.clone());
        });
    }

    pub fn execute(&mut self, event: Event) {
        if let Some(button_args) = event.button_args() {
            match button_args.state {
                ButtonState::Press => {
                    // prevent repeated key pressed events when key is hold down
                    if !self
                        .resources
                        .button_storage
                        .lock()
                        .unwrap()
                        .contains(&button_args.button)
                    {
                        self.resources
                            .button_storage
                            .lock()
                            .unwrap()
                            .insert(button_args.button);
                        self.update(event);
                    }
                }
                ButtonState::Release => {
                    self.resources
                        .button_storage
                        .lock()
                        .unwrap()
                        .remove(&button_args.button);
                    self.update(event);
                }
            }
        } else if event.render_args().is_some() {
            self.draw(event);
        } else {
            self.update(event);
        }
    }
}
