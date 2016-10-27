extern crate tic_tac_toe;
use tic_tac_toe::game_board::game_board::GameBoard;
//use tic_tac_toe::game_board::basic_board_traits::BasicBoard;

pub const FILL: usize = 0;
pub const TEST_MOVE: usize = 1;

pub fn board_3x3() -> GameBoard<usize> {
    GameBoard::new(3, FILL)
}

pub fn board_4x4() -> GameBoard<usize> {
    GameBoard::new(4, FILL)
}
