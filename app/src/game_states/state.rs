pub use graphics::{Context, Graphics};
pub use opengl_graphics::{GlGraphics, Texture};
pub use piston::input::Event;

pub enum StateTransition {
    Push(Box<dyn State>),
    Pop,
    Switch(Box<dyn State>),
    Clear,
    None,
}

pub struct StateStackEvent(pub StateTransition, pub bool);

pub trait State {
    fn event(&mut self, event: &Event) -> StateStackEvent {
        StateStackEvent(StateTransition::None, true)
    }

    fn update(&mut self, dt: f64) -> StateStackEvent {
        StateStackEvent(StateTransition::None, true)
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {}
}

pub struct StateManager {
    stack: Vec<Box<dyn State>>,
    pending_transitions: Vec<StateTransition>,
}

impl StateManager {
    pub fn new(state: Box<dyn State>) -> StateManager {
        StateManager {
            stack: vec![state],
            pending_transitions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn event(&mut self, event: &Event) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.event(event);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions();
    }

    pub fn update(&mut self, dt: f64) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.update(dt);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions();
    }

    pub fn draw(&self, c: &Context, g: &mut GlGraphics) {
        for state in self.stack.iter() {
            state.draw(c, g);
        }
    }

    fn apply_pending_transitions(&mut self) {
        self.pending_transitions.reverse();

        while let Some(transition) = self.pending_transitions.pop() {
            match transition {
                StateTransition::Push(state) => {
                    self.stack.push(state);
                }
                StateTransition::Pop => {
                    self.stack.pop();
                }
                StateTransition::Switch(state) => {
                    self.stack.pop();
                    self.stack.push(state);
                }
                StateTransition::Clear => {
                    self.stack.clear();
                }
                StateTransition::None => {}
            }
        }
    }
}
