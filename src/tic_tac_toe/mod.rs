use super::Game;

use glutin::{ GlWindow, Event};
use nanovg;
use nanovg::{Frame, Color, StrokeOptions};

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub const DRAW_SIZE: f32 = 600.0;

pub fn board_color() -> Color{
    Color::new(1.0, 1.0, 1.0, 1.0)
}

pub fn x_color() -> Color{
    Color::new(0.0, 0.0, 1.0, 1.0)
}

pub fn o_color() -> Color{
    Color::new(1.0, 0.0, 0.0, 1.0)
}

fn draw_line(frame: &Frame, start: (f32, f32), end: (f32, f32), color: Color, width: f32){
    frame.path(|path|{
        path.move_to(start);
        path.line_to(end);
        path.stroke(color, StrokeOptions{width , ..Default::default()});
    }, Default::default());
}

pub struct TicTacToe {
    board: Board,
}

impl TicTacToe{
    pub fn new() -> Self {
        TicTacToe{board: Board::new()}
    }
}

impl Game for TicTacToe {
    type ContextType = nanovg::Context;
    type WindowType = GlWindow;

    fn handle_event(&mut self, event: &Event) {
    }

    fn animate(&mut self, dt: f32) {}

    fn draw(&self, context: &Self::ContextType, window: &Self::WindowType) {
        let (width, height) = window.get_inner_size().unwrap();
        let pos = ((width as f32 - DRAW_SIZE) / 2.0, (height as f32 - DRAW_SIZE) / 2.0);
        let tile_size = (DRAW_SIZE / self.board.width() as f32, DRAW_SIZE / self.board.height() as f32);

        // Draw board
        context.frame((width as i32, height as i32), window.hidpi_factor(), |frame| {

            // Horizontal lines
            for i in 0..=self.board.width() {
                let origin = (pos.0 + tile_size.0 * i as f32, pos.1);
                let end = (origin.0, origin.1 + DRAW_SIZE);
                draw_line(&frame, origin, end, board_color(), 3.0);
            }

            // Vertical lines
            for i in 0..=self.board.height() {
                let origin = (pos.0, pos.1 + tile_size.1 * i as f32);
                let end = (origin.0 + DRAW_SIZE, origin.1);
                draw_line(&frame, origin, end, board_color(), 3.0);
            }

            // Draw tiles
            for x in 0..self.board.width(){
                for y in 0..self.board.height(){
                    let center = (x as f32 * tile_size.0 + pos.0 + (tile_size.0 / 2.0), y as f32 * tile_size.1 + pos.1 + (tile_size.1 / 2.0));
                    let dim = (tile_size.0 * 0.75, tile_size.1 * 0.75);

                    self.board.get_tile((x, y)).draw(&frame, center, dim, 12.0);
                }
            }
        });


    }
}

#[derive(Clone, Copy)]
enum SquareState {
    Empty,
    X,
    O,
}

impl SquareState {
    pub fn draw(self, frame: &Frame, center: (f32, f32), (width, height): (f32, f32), stroke: f32) {
        match self {
            // Draw O
            SquareState::O => {
                frame.path(|path| {
                    path.ellipse(center, width / 2.0, height / 2.0);
                    path.stroke(o_color(), StrokeOptions{width: stroke, ..Default::default()});
                }, Default::default());
            },

            // Draw X
            SquareState::X => {
                let top_left = (center.0 - width / 2.0, center.1 - height / 2.0);
                let bottom_left = (center.0 - width / 2.0, center.1 + height / 2.0);
                let top_right = (center.0 + width / 2.0, center.1 - height / 2.0);
                let bottom_right = (center.0 + width / 2.0, center.1 + height / 2.0);

                draw_line(&frame, top_left, bottom_right, x_color(), stroke);
                draw_line(&frame, top_right, bottom_left, x_color(), stroke);
            },
            _ => (),
        }
    }
}

struct Board {
    pub tiles: [SquareState; BOARD_SIZE],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            tiles: [SquareState::Empty; BOARD_SIZE],
        };

        for i in 0..3 {
            board.set_tile((i, i), SquareState::O);
        }

        for i in 0..3{
            board.set_tile((2 - i, i), SquareState::X);
        }

        board
    }

    pub fn size(&self) -> (usize, usize){
        (BOARD_WIDTH, BOARD_HEIGHT)
    }

    pub fn width(&self) -> usize{
        BOARD_WIDTH
    }

    pub fn height(&self) -> usize{
        BOARD_HEIGHT
    }

    pub fn get_tile(&self, (x, y): (usize, usize)) -> SquareState{
        self.tiles[x * BOARD_HEIGHT + y]
    }

    pub fn set_tile(&mut self, (x, y): (usize, usize), state: SquareState){
        self.tiles[x * BOARD_HEIGHT + y] = state;
    }
}
