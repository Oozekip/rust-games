use std::fmt;
use nanovg::{Color, Frame, StrokeOptions};
use super::utils;

#[derive(Clone, Copy)]
pub enum SquareState {
    Empty,
    X,
    O,
}

impl fmt::Display for SquareState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            SquareState::X => 'X',
            SquareState::O => 'O',
            _ => '_',
        };

        write!(f, "{}", character)
    }
}

impl SquareState {
    pub fn draw(
        self,
        frame: &Frame,
        center: (f32, f32),
        (width, height): (f32, f32),
        stroke: f32,
        x_color: Color,
        o_color: Color,
    ) {
        match self {
            // Draw O
            SquareState::O => {
                frame.path(
                    |path| {
                        path.ellipse(center, width / 2.0, height / 2.0);
                        path.stroke(
                            o_color,
                            StrokeOptions {
                                width: stroke,
                                ..Default::default()
                            },
                        );
                    },
                    Default::default(),
                );
            }

            // Draw X
            SquareState::X => {
                let top_left = (center.0 - width / 2.0, center.1 - height / 2.0);
                let bottom_left = (center.0 - width / 2.0, center.1 + height / 2.0);
                let top_right = (center.0 + width / 2.0, center.1 - height / 2.0);
                let bottom_right = (center.0 + width / 2.0, center.1 + height / 2.0);

                utils::draw_line(&frame, top_left, bottom_right, x_color, stroke);
                utils::draw_line(&frame, top_right, bottom_left, x_color, stroke);
            }
            _ => (),
        }
    }
}
