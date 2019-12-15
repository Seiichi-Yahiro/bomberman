use crate::traits::game_loop_event::*;
use crate::world::World;

pub trait GameState: EventHandler<StateStackEvent> + Updatable<StateStackEvent> + Drawable {
    fn on_create(&mut self, _world: &mut World) {}
    fn on_destroy(&self, _world: &mut World) {}
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
    pub fn new(mut state: Box<dyn GameState>, world: &mut World) -> StateManager {
        state.on_create(world);

        StateManager {
            stack: vec![state],
            pending_transitions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn apply_pending_transitions(&mut self, world: &mut World) {
        self.pending_transitions.reverse();

        while let Some(transition) = self.pending_transitions.pop() {
            match transition {
                StateTransition::Push(state) => {
                    self.push(state, world);
                }
                StateTransition::Pop => {
                    self.pop(world);
                }
                StateTransition::Switch(state) => {
                    self.pop(world);
                    self.push(state, world);
                }
                StateTransition::Clear => {
                    while let Some(old_state) = self.stack.pop() {
                        old_state.on_destroy(world);
                    }
                }
                StateTransition::None => {}
            }
        }
    }

    fn push(&mut self, mut state: Box<dyn GameState>, world: &mut World) {
        state.on_create(world);
        self.stack.push(state)
    }

    fn pop(&mut self, world: &mut World) {
        if let Some(old_state) = self.stack.pop() {
            old_state.on_destroy(world);
        }
    }
}

impl EventHandler for StateManager {
    fn handle_event(&mut self, world: &mut World, event: &Event) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.handle_event(world, event);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions(world);
    }
}

impl Updatable for StateManager {
    fn update(&mut self, world: &mut World, dt: f64) {
        for state in self.stack.iter_mut().rev() {
            let StateStackEvent(transition, should_pass_down) = state.update(world, dt);
            self.pending_transitions.push(transition);

            if !should_pass_down {
                break;
            }
        }

        self.apply_pending_transitions(world);
    }
}

impl Drawable for StateManager {
    fn draw(&self, world: &World, c: &Context, g: &mut GlGraphics) {
        self.stack.iter().for_each(|state| state.draw(world, c, g))
    }
}
