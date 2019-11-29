use crate::traits::game_loop_event::*;

pub enum StateTransition {
    Push(Box<dyn GameLoopEvent<StateStackEvent>>),
    Pop,
    Switch(Box<dyn GameLoopEvent<StateStackEvent>>),
    Clear,
    None,
}

pub struct StateStackEvent(pub StateTransition, pub bool);

impl Default for StateStackEvent {
    fn default() -> Self {
        StateStackEvent(StateTransition::None, true)
    }
}

pub struct StateManager {
    stack: Vec<Box<dyn GameLoopEvent<StateStackEvent>>>,
    pending_transitions: Vec<StateTransition>,
}

impl StateManager {
    pub fn new(state: Box<dyn GameLoopEvent<StateStackEvent>>) -> StateManager {
        StateManager {
            stack: vec![state],
            pending_transitions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
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

impl GameLoopEvent<()> for StateManager {
    fn event(&mut self, event: &Event) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.event(event);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions();
    }

    fn update(&mut self, dt: f64) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.update(dt);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions();
    }

    fn draw(&self, c: &Context, g: &mut GlGraphics) {
        for state in self.stack.iter() {
            state.draw(c, g);
        }
    }
}
