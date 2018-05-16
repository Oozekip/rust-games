extern crate glutin;
extern crate nanovg;

use glutin::WindowEvent;

pub trait Game {
    /// Type of the context that is passed to draw
    type ContextType;

    /// Type of the window that is passed to draw
    type WindowType;

    /// Passes a window event to the game to handle
    fn handle_event(&mut self, event: &WindowEvent);

    /// Called to update the game after a certain length of time
    fn animate(&mut self, dt: f32);

    /// Called every frame to draw the game
    fn draw(&self, context: &Self::ContextType, window: &Self::WindowType);
}

pub mod tic_tac_toe;
