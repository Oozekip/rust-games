extern crate glutin;
extern crate nanovg;

use glutin::{Event};

pub trait Game {
    type ContextType;
    type WindowType;

    fn handle_event(&mut self, event: &Event);
    fn animate(&mut self, dt: f32);
    fn draw(&self, context: &Self::ContextType, window: &Self::WindowType); // TODO: Determine args?
}

pub mod tic_tac_toe;
