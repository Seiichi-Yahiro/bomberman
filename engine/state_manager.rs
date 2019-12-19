use crate::asset_storage::AssetStorage;
use crate::game_state_builder::GameStateBuilder;
use crate::traits::game_loop_event::*;

pub trait GameState {
    fn handle_event(&mut self, state_context: &mut StateContext, event: &Event) -> bool;
    fn update(&mut self, state_context: &mut StateContext, dt: f64) -> bool;
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics);
}

pub struct StateContext<'a, 's> {
    pub asset_storage: &'a AssetStorage,
    pub request_state_transition: &'s mut dyn FnMut(StateTransition),
}

pub enum StateTransition {
    Push(GameStateBuilder),
    Pop,
    Switch(GameStateBuilder),
    Clear,
}

pub struct StateManager {
    stack: Vec<Box<dyn GameState>>,
}

impl StateManager {
    pub fn new(
        game_state_builder: GameStateBuilder,
        asset_storage: &mut AssetStorage,
    ) -> StateManager {
        StateManager {
            stack: vec![(game_state_builder.build)(asset_storage)],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn apply_pending_transitions(
        &mut self,
        mut pending_transitions: Vec<StateTransition>,
        asset_storage: &mut AssetStorage,
    ) {
        pending_transitions.reverse();

        while let Some(transition) = pending_transitions.pop() {
            match transition {
                StateTransition::Push(builder) => {
                    self.stack.push((builder.build)(asset_storage));
                }
                StateTransition::Pop => {
                    self.stack.pop();
                }
                StateTransition::Switch(builder) => {
                    self.stack.pop();
                    self.stack.push((builder.build)(asset_storage));
                }
                StateTransition::Clear => self.stack.clear(),
            }
        }
    }

    pub fn handle_event(&mut self, asset_storage: &mut AssetStorage, event: &Event) {
        let mut pending_transitions: Vec<StateTransition> = vec![];

        let mut state_context = StateContext {
            asset_storage,
            request_state_transition: &mut |state_transition| {
                pending_transitions.push(state_transition)
            },
        };

        for state in self.stack.iter_mut().rev() {
            // if should not pass down
            if !state.handle_event(&mut state_context, event) {
                break;
            }
        }

        self.apply_pending_transitions(pending_transitions, asset_storage);
    }

    pub fn update(&mut self, asset_storage: &mut AssetStorage, dt: f64) {
        let mut pending_transitions: Vec<StateTransition> = vec![];

        let mut state_context = StateContext {
            asset_storage,
            request_state_transition: &mut |state_transition| {
                pending_transitions.push(state_transition)
            },
        };

        for state in self.stack.iter_mut().rev() {
            // if should not pass down
            if !state.update(&mut state_context, dt) {
                break;
            }
        }

        self.apply_pending_transitions(pending_transitions, asset_storage);
    }
}

impl Drawable for StateManager {
    fn draw(&self, transform: Matrix2d, g: &mut GlGraphics) {
        self.stack.iter().for_each(|state| state.draw(transform, g))
    }
}
