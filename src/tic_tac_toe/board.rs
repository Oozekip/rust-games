use super::tile::SquareState;

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub struct Board {
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
