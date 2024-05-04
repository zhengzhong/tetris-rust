use crate::common::{GamePad, GameUI};
use crate::states::{create_state, State, StateName};

pub struct Tetris {
    state: Box<dyn State>,
}

impl Tetris {
    pub fn new() -> Self {
        Self {
            state: create_state(StateName::Intro),
        }
    }

    pub fn start_loop(&mut self) {
        self.state.start_loop();
    }

    pub fn process_input(&mut self, pad: &dyn GamePad) {
        self.state.process_input(pad);
    }

    pub fn update(&mut self) {
        self.state.update();
    }

    pub fn draw(&self, ui: &mut dyn GameUI) {
        self.state.draw(ui);
    }

    pub fn end_loop(&mut self) {
        let next_state_name = self.state.end_loop();
        match next_state_name {
            Some(name) => self.state = create_state(name),
            None => (),
        }
    }
}
