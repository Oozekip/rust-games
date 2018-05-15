use super::Game;

use std::fmt;

use glutin::{ElementState, GlWindow, MouseButton, WindowEvent};
use nanovg;
use nanovg::{Color, Frame, StrokeOptions};

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

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

fn draw_line(frame: &Frame, start: (f32, f32), end: (f32, f32), color: Color, width: f32) {
    frame.path(
        |path| {
            path.move_to(start);
            path.line_to(end);
            path.stroke(
                color,
                StrokeOptions {
                    width,
                    ..Default::default()
                },
            );
        },
        Default::default(),
    );
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
        draw_line(&frame, origin, end, color, stroke);
    }

    // Vertical lines
    for i in 0..=cols {
        let origin = (x, y + tile_size.1 * i as f32);
        let end = (origin.0 + width, origin.1);
        draw_line(&frame, origin, end, color, stroke);
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

#[derive(Clone, Copy)]
enum SquareState {
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

                draw_line(&frame, top_left, bottom_right, x_color, stroke);
                draw_line(&frame, top_right, bottom_left, x_color, stroke);
            }
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
        Self {
            tiles: [SquareState::Empty; BOARD_SIZE],
        }
    }

    pub fn width(&self) -> usize {
        BOARD_WIDTH
    }

    pub fn height(&self) -> usize {
        BOARD_HEIGHT
    }

    pub fn screen_pos(&self, (width, height): (u32, u32)) -> (u32, u32) {
        let (x, y) = self.screen_size((width, height));

        ((width - x) / 2, (height - y) / 2)
    }

    pub fn screen_size(&self, (width, height): (u32, u32)) -> (u32, u32) {
        let draw_size = u32::min(width, height);
        (draw_size, draw_size)
    }

    pub fn clicked_tile(
        &self,
        screen_size: (u32, u32),
        (x, y): (u32, u32),
    ) -> Option<(usize, usize)> {
        let (screen_x, screen_y) = self.screen_pos(screen_size);
        let (width, height) = self.screen_size(screen_size);
        let offset_x = i64::from(x) - i64::from(screen_x);
        let offset_y = i64::from(y) - i64::from(screen_y);

        if (0 <= offset_x && offset_x < i64::from(width))
            && (0 <= offset_y && offset_y < i64::from(height))
        {
            // offset / tile size
            let tile_width = width / self.width() as u32;
            let tile_height = height / self.height() as u32;

            Some((
                offset_x as usize / tile_width as usize,
                offset_y as usize / tile_height as usize,
            ))
        } else {
            None
        }
    }

    pub fn get_tile(&self, (x, y): (usize, usize)) -> SquareState {
        self.tiles[x * BOARD_HEIGHT + y]
    }

    pub fn set_tile(&mut self, (x, y): (usize, usize), state: SquareState) {
        self.tiles[x * BOARD_HEIGHT + y] = state;
    }

    pub fn check_winner(&self) -> Option<Option<SquareState>> {
        let mut rows = [0; BOARD_WIDTH];
        let mut cols = [0; BOARD_HEIGHT];
        let mut right_diag = 0;
        let mut left_diag = 0;
        let mut filled = 0;

        for x in 0..rows.len() {
            for y in 0..cols.len() {
                // Convert state to value
                let tile = match self.get_tile((x, y)) {
                    SquareState::X => 1,
                    SquareState::O => -1,
                    _ => 0,
                };
                rows[x] += tile;
                cols[y] += tile;

                // Checking left diagonal
                if x == y {
                    left_diag += tile;
                }

                // Checking right diagonal
                let mid_x = (BOARD_WIDTH / 2) as i32;
                let mid_y = (BOARD_HEIGHT / 2) as i32;

                if (x as i32 - mid_x) + (y as i32 - mid_y) == 0 {
                    right_diag += tile;
                }

                filled += i32::abs(tile);
            }
        }

        let abs_max = |acc, &x| {
            if i32::abs(x) > i32::abs(acc) {
                x
            } else {
                acc
            }
        };

        let mut lead = rows.into_iter().fold(0, abs_max);
        lead = cols.into_iter().fold(lead, abs_max);
        lead = abs_max(lead, &left_diag);
        lead = abs_max(lead, &right_diag);

        match (lead, filled) {
            (3, _) => Some(Some(SquareState::X)),
            (-3, _) => Some(Some(SquareState::O)),
            (_, 9) => Some(None),
            _ => None,
        }
    }
}

enum GameState {
    GameOver(Option<SquareState>), // Who won the game (tie if None)
    Running(SquareState),          // Whose turn it is
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameState::Running(state) => write!(f, "{}'s turn", state),
            GameState::GameOver(winner) => if let Some(winner) = winner {
                write!(f, "{} wins", winner)
            } else {
                write!(f, "Draw")
            },
        }
    }
}
