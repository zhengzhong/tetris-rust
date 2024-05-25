use crate::conf::TetrisSettings;
use crate::{GamePad, GameUI};

pub enum StateName {
    Intro,
    Ongoing,
}

pub trait State {
    fn start_loop(&mut self);

    fn process_input(&mut self, pad: &dyn GamePad);

    fn update(&mut self);

    fn draw(&self, ui: &mut dyn GameUI);

    fn end_loop(&self) -> Option<StateName>;
}

mod intro;
mod ongoing;

use intro::Intro;
use ongoing::Ongoing;

pub fn create_state<'a>(name: StateName, settings: &'a TetrisSettings) -> Box<dyn State + 'a> {
    match name {
        StateName::Intro => Box::new(Intro::new(settings)),
        StateName::Ongoing => Box::new(Ongoing::new(settings)),
    }
}
