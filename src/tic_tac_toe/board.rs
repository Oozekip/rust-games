use nanovg::{Color, Frame};

use super::tile::Tile;
use super::utils;

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub struct Board {
    pub tiles: [Tile; BOARD_SIZE],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::Empty; BOARD_SIZE],
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

    pub fn draw_board(
        &self,
        frame: &Frame,
        screen_size: (u32, u32),
        board_color: Color,
        stroke: f32,
    ) {
        let dims = self.screen_size(screen_size);
        let pos = self.screen_pos(screen_size);

        let fwidth = self.width() as f32;
        let fheight = self.height() as f32;
        let (draw_width, draw_height) = (dims.0 as f32, dims.1 as f32);
        let tile_size = (draw_width / fwidth, draw_height / fheight);
        let (x, y) = (pos.0 as f32, pos.1 as f32);

        // Horizontal lines
        for i in 0..=self.width() {
            let origin = (x + tile_size.0 * i as f32, y);
            let end = (origin.0, origin.1 + draw_height);
            utils::draw_line(&frame, origin, end, board_color, stroke);
        }

        // Vertical lines
        for i in 0..=self.width() {
            let origin = (x, y + tile_size.1 * i as f32);
            let end = (origin.0 + draw_width, origin.1);
            utils::draw_line(&frame, origin, end, board_color, stroke);
        }
    }

    pub fn draw_tiles(
        &self,
        frame: &Frame,
        screen_size: (u32, u32),
        size_ratio: f32,
        x_color: Color,
        o_color: Color,
        stroke: f32,
    ) {
        let dims = self.screen_size(screen_size);
        let pos = self.screen_pos(screen_size);

        let fwidth = self.width() as f32;
        let fheight = self.height() as f32;
        let (draw_width, draw_height) = (dims.0 as f32, dims.1 as f32);
        let (x, y) = (pos.0 as f32, pos.1 as f32);
        let tile_size = (draw_width / fwidth, draw_height / fheight);

        // Tiles
        // Draw tiles
        for i in 0..self.width() {
            for j in 0..self.height() {
                let center = (
                    i as f32 * tile_size.0 + x + (tile_size.0 / 2.0),
                    j as f32 * tile_size.1 + y + (tile_size.1 / 2.0),
                );
                let dim = (tile_size.0 * 0.75, tile_size.1 * size_ratio);

                self.get_tile((i, j))
                    .draw(&frame, center, dim, stroke, x_color, o_color);
            }
        }
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

    pub fn get_tile(&self, (x, y): (usize, usize)) -> Tile {
        self.tiles[x * BOARD_HEIGHT + y]
    }

    pub fn set_tile(&mut self, (x, y): (usize, usize), state: Tile) {
        self.tiles[x * BOARD_HEIGHT + y] = state;
    }

    pub fn check_winner(&self) -> Option<Option<Tile>> {
        let mut rows = [0; BOARD_WIDTH];
        let mut cols = [0; BOARD_HEIGHT];
        let mut right_diag = 0;
        let mut left_diag = 0;
        let mut filled = 0;

        for (x, row) in rows.iter_mut().enumerate() {
            for (y, col) in cols.iter_mut().enumerate() {
                // Convert state to value
                let tile = match self.get_tile((x, y)) {
                    Tile::X => 1,
                    Tile::O => -1,
                    _ => 0,
                };
                *row += tile;
                *col += tile;

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
            (3, _) => Some(Some(Tile::X)),
            (-3, _) => Some(Some(Tile::O)),
            (_, 9) => Some(None),
            _ => None,
        }
    }
}
