use std::fmt;

use super::tile::SquareState;

pub enum GameState {
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
