use super::Game;

use glutin::{ElementState, GlWindow, MouseButton, WindowEvent};
use nanovg;
use nanovg::Color;

mod board;
mod state;
mod tile;
mod utils;

use self::board::Board;
use self::state::GameState;
use self::tile::Tile;

pub const BOARD_STROKE: f32 = 3.0;
pub const TILE_STROKE: f32 = 12.0;
pub const TILE_SIZE_RATIO: f32 = 0.75;

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
            state: GameState::Running(Tile::X),
            board: Default::default(),
        }
    }

    pub fn reset_game(&mut self) {
        self.board = Default::default();
        self.state = GameState::Running(Tile::X);
    }

    fn play_turn(&mut self, clicked: (usize, usize)) {
        match self.state {
            GameState::GameOver(..) => self.reset_game(), // Reset the game
            GameState::Running(turn) => {
                // Play a turn
                if let Tile::Empty = self.board.get_tile(clicked) {
                    self.board.set_tile(clicked, turn);

                    if let Some(winner) = self.board.check_winner() {
                        self.state = GameState::GameOver(winner);
                    } else if let Tile::X = turn {
                        self.state = GameState::Running(Tile::O);
                    } else {
                        self.state = GameState::Running(Tile::X);
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

        window.set_title(&format!("{}", self.state).as_str());
        let (board_color, x_color, o_color) = match self.state {
            GameState::GameOver(winner) => match winner {
                Some(winner) => match winner {
                    Tile::X => (x_color(), x_color(), x_color()),
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
                self.board
                    .draw_board(&frame, self.screen_size, board_color, BOARD_STROKE);
                self.board.draw_tiles(
                    &frame,
                    self.screen_size,
                    TILE_SIZE_RATIO,
                    x_color,
                    o_color,
                    TILE_STROKE,
                );
            },
        );
    }
}
