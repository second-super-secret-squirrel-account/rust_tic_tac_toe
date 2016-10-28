extern crate tic_tac_toe;
use tic_tac_toe::grid::Grid;
//use tic_tac_toe::game_board::game_board_traits::GameBoard;

pub const FILL: usize = 999;
pub const TEST_MOVE: usize = 1;

pub fn board_3x3() -> Grid<usize> {
    Grid::new(3, 3, FILL)
}

pub fn board_4x4() -> Grid<usize> {
    Grid::new(4, 4, FILL)
}
