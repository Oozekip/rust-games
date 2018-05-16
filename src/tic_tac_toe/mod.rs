use super::Game;

use glutin::{ElementState, GlWindow, MouseButton, WindowEvent};
use nanovg;
use nanovg::{Color, Frame};

mod board;
mod tile;
mod utils;
mod state;

use self::tile::SquareState;
use self::board::Board;
use self::state::GameState;

pub fn board_color() -> Color {
    Color::new(1.0, 1.0, 1.0, 1.0)
}

pub fn x_color() -> Color {
    Color::new(0.0, 0.0, 1.0, 1.0)
}

pub fn o_color() -> Color {
    Color::new(1.0, 0.0, 0.0, 1.0)
}

pub fn draw_color() -> Color {
    Color::new(0.75, 0.75, 0.75, 1.0)
}

fn draw_board(
    frame: &Frame,
    (rows, cols): (usize, usize),
    (x, y): (f32, f32),
    (width, height): (f32, f32),
    color: Color,
    stroke: f32,
) {
    let tile_size = (width / rows as f32, height / cols as f32);
    // Horizontal lines
    for i in 0..=rows {
        let origin = (x + tile_size.0 * i as f32, y);
        let end = (origin.0, origin.1 + height);
        utils::draw_line(&frame, origin, end, color, stroke);
    }

    // Vertical lines
    for i in 0..=cols {
        let origin = (x, y + tile_size.1 * i as f32);
        let end = (origin.0 + width, origin.1);
        utils::draw_line(&frame, origin, end, color, stroke);
    }
}

pub struct TicTacToe {
    board: Board,
    cursor_pos: (u32, u32),
    screen_size: (u32, u32),
    state: GameState,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            cursor_pos: (0, 0),
            screen_size: (0, 0),
            state: GameState::Running(SquareState::X),
            board: Default::default(),
        }
    }

    pub fn reset_game(&mut self) {
        self.board = Default::default();
        self.state = GameState::Running(SquareState::X);
    }

    fn play_turn(&mut self, clicked: (usize, usize)) {
        match self.state {
            GameState::GameOver(..) => self.reset_game(), // Reset the game
            GameState::Running(turn) => {
                // Play a turn
                if let SquareState::Empty = self.board.get_tile(clicked) {
                    self.board.set_tile(clicked, turn);

                    if let Some(winner) = self.board.check_winner() {
                        self.state = GameState::GameOver(winner);
                    } else if let SquareState::X = turn {
                        self.state = GameState::Running(SquareState::O);
                    } else {
                        self.state = GameState::Running(SquareState::X);
                    }
                }
            }
        }
    }
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for TicTacToe {
    type ContextType = nanovg::Context;
    type WindowType = GlWindow;

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(w, h) => self.screen_size = (*w, *h),
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = (position.0 as u32, position.1 as u32);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if let MouseButton::Left = button {
                    if let ElementState::Pressed = state {
                        if let Some(clicked) =
                            self.board.clicked_tile(self.screen_size, self.cursor_pos)
                        {
                            self.play_turn(clicked);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn animate(&mut self, _dt: f32) {}

    fn draw(&self, context: &Self::ContextType, window: &Self::WindowType) {
        let (width, height) = self.screen_size;
        let pos_int = self.board.screen_pos((width as u32, height as u32));
        let pos = (pos_int.0 as f32, pos_int.1 as f32);
        let draw_size = u32::min(width, height) as f32;
        let tile_size = (
            draw_size / self.board.width() as f32,
            draw_size / self.board.height() as f32,
        );

        window.set_title(&format!("{}", self.state).as_str());
        let (board_color, x_color, o_color) = match self.state {
            GameState::GameOver(winner) => match winner {
                Some(winner) => match winner {
                    SquareState::X => (x_color(), x_color(), x_color()),
                    _ => (o_color(), o_color(), o_color()),
                },
                _ => (draw_color(), draw_color(), draw_color()),
            },

            _ => (board_color(), x_color(), o_color()),
        };

        // Draw board
        context.frame(
            (width as i32, height as i32),
            window.hidpi_factor(),
            |frame| {
                draw_board(
                    &frame,
                    (self.board.width(), self.board.height()),
                    pos,
                    (draw_size, draw_size),
                    board_color,
                    3.0,
                );

                // Draw tiles
                for x in 0..self.board.width() {
                    for y in 0..self.board.height() {
                        let center = (
                            x as f32 * tile_size.0 + pos.0 + (tile_size.0 / 2.0),
                            y as f32 * tile_size.1 + pos.1 + (tile_size.1 / 2.0),
                        );
                        let dim = (tile_size.0 * 0.75, tile_size.1 * 0.75);

                        self.board
                            .get_tile((x, y))
                            .draw(&frame, center, dim, 12.0, x_color, o_color);
                    }
                }
            },
        );
    }
}
