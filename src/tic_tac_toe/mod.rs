use super::Game;

use glutin::{ElementState, GlWindow, MouseButton, WindowEvent};
use nanovg;
use nanovg::{Color, Frame, StrokeOptions};

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub const DRAW_SIZE: f32 = 600.0;

pub fn board_color() -> Color {
    Color::new(1.0, 1.0, 1.0, 1.0)
}

pub fn x_color() -> Color {
    Color::new(0.0, 0.0, 1.0, 1.0)
}

pub fn o_color() -> Color {
    Color::new(1.0, 0.0, 0.0, 1.0)
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
        let end = (origin.0, origin.1 + DRAW_SIZE);
        draw_line(&frame, origin, end, color, stroke);
    }

    // Vertical lines
    for i in 0..=cols {
        let origin = (x, y + tile_size.1 * i as f32);
        let end = (origin.0 + DRAW_SIZE, origin.1);
        draw_line(&frame, origin, end, color, stroke);
    }
}

pub struct TicTacToe {
    board: Board,
    cursor_pos: (u32, u32),
    screen_size: (u32, u32),
    turn: SquareState,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
            cursor_pos: (0, 0),
            screen_size: (0, 0),
            turn: SquareState::X,
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
                //println!("Mouse moved to {:?}", self.cursor_pos);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if let MouseButton::Left = button {
                    if let ElementState::Pressed = state {
                        if let Some(clicked) =
                            self.board.clicked_tile(self.screen_size, self.cursor_pos)
                        {
                            if let SquareState::Empty = self.board.get_tile(clicked) {
                                self.board.set_tile(clicked, self.turn);

                                if let SquareState::X = self.turn {
                                    self.turn = SquareState::O;
                                } else {
                                    self.turn = SquareState::X;
                                }
                            }
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
        let tile_size = (
            DRAW_SIZE / self.board.width() as f32,
            DRAW_SIZE / self.board.height() as f32,
        );

        // Draw board
        context.frame(
            (width as i32, height as i32),
            window.hidpi_factor(),
            |frame| {
                draw_board(
                    &frame,
                    (self.board.width(), self.board.height()),
                    pos,
                    (DRAW_SIZE, DRAW_SIZE),
                    board_color(),
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

                        self.board.get_tile((x, y)).draw(&frame, center, dim, 12.0);
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

impl SquareState {
    pub fn draw(self, frame: &Frame, center: (f32, f32), (width, height): (f32, f32), stroke: f32) {
        match self {
            // Draw O
            SquareState::O => {
                frame.path(
                    |path| {
                        path.ellipse(center, width / 2.0, height / 2.0);
                        path.stroke(
                            o_color(),
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

                draw_line(&frame, top_left, bottom_right, x_color(), stroke);
                draw_line(&frame, top_right, bottom_left, x_color(), stroke);
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
        let (x, y) = self.screen_size();

        ((width - x) / 2, (height - y) / 2)
    }

    pub fn screen_size(&self) -> (u32, u32) {
        (DRAW_SIZE as u32, DRAW_SIZE as u32)
    }

    pub fn clicked_tile(
        &self,
        screen_size: (u32, u32),
        (x, y): (u32, u32),
    ) -> Option<(usize, usize)> {
        let (screen_x, screen_y) = self.screen_pos(screen_size);
        let (width, height) = self.screen_size();
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
}
