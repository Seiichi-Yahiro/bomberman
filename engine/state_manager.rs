use crate::asset_storage::AssetStorage;
use crate::traits::game_loop_event::*;

pub trait GameState: EventHandler<StateStackEvent> + Updatable<StateStackEvent> + Drawable {
    fn on_create(&mut self, _asset_storage: &mut AssetStorage) {}
    fn on_destroy(&self, _asset_storage: &mut AssetStorage) {}
}

pub enum StateTransition {
    Push(Box<dyn GameState>),
    Pop,
    Switch(Box<dyn GameState>),
    Clear,
    None,
}

pub struct StateStackEvent(pub StateTransition, pub bool);

pub struct StateManager {
    stack: Vec<Box<dyn GameState>>,
    pending_transitions: Vec<StateTransition>,
}

impl StateManager {
    pub fn new(mut state: Box<dyn GameState>, asset_storage: &mut AssetStorage) -> StateManager {
        state.on_create(asset_storage);

        StateManager {
            stack: vec![state],
            pending_transitions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn apply_pending_transitions(&mut self, asset_storage: &mut AssetStorage) {
        self.pending_transitions.reverse();

        while let Some(transition) = self.pending_transitions.pop() {
            match transition {
                StateTransition::Push(state) => {
                    self.push(state, asset_storage);
                }
                StateTransition::Pop => {
                    self.pop(asset_storage);
                }
                StateTransition::Switch(state) => {
                    self.pop(asset_storage);
                    self.push(state, asset_storage);
                }
                StateTransition::Clear => {
                    while let Some(old_state) = self.stack.pop() {
                        old_state.on_destroy(asset_storage);
                    }
                }
                StateTransition::None => {}
            }
        }
    }

    fn push(&mut self, mut state: Box<dyn GameState>, asset_storage: &mut AssetStorage) {
        state.on_create(asset_storage);
        self.stack.push(state)
    }

    fn pop(&mut self, asset_storage: &mut AssetStorage) {
        if let Some(old_state) = self.stack.pop() {
            old_state.on_destroy(asset_storage);
        }
    }
}

impl EventHandler for StateManager {
    fn handle_event(&mut self, asset_storage: &mut AssetStorage, event: &Event) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) =
                state.handle_event(asset_storage, event);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions(asset_storage);
    }
}

impl Updatable for StateManager {
    fn update(&mut self, asset_storage: &mut AssetStorage, dt: f64) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.update(asset_storage, dt);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions(asset_storage);
    }
}

impl Drawable for StateManager {
    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        self.stack.iter().for_each(|state| state.draw(c, g))
    }
}
