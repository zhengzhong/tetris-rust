use crate::common::{GamePad, GameUI};
use crate::conf::TetrisSettings;
use crate::states::{create_state, State, StateName};

pub struct Tetris<'a> {
    settings: &'a TetrisSettings,
    state: Box<dyn State + 'a>,
}

impl<'a> Tetris<'a> {
    pub fn new(settings: &'a TetrisSettings) -> Self {
        Self {
            settings,
            state: create_state(StateName::Intro, settings),
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
        if let Some(state_name) = next_state_name {
            self.state = create_state(state_name, self.settings);
        }
    }
}
