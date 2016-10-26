extern crate tic_tac_toe;
use tic_tac_toe::game_board::game_board::GameBoard;
use tic_tac_toe::game_board::properties_traits::HasProperties;

static FILL: usize = 0;

fn board_3x3() -> GameBoard<usize> {
    GameBoard::new(3, FILL)
}

fn board_4x4() -> GameBoard<usize> {
    GameBoard::new(4, FILL)
}

#[test]
fn trait_size_returns_9_on_3x3_board() {
    let test_board = board_3x3();
    assert_eq!(9, test_board.size());
}

#[test]
fn row_size_returns_3_on_3x3_board() {
    let test_board = board_3x3();
    assert_eq!(3, test_board.row_size());
}

#[test]
fn row_size_returns_4_on_4x4_board() {
    let test_board = board_4x4();
    assert_eq!(4, test_board.row_size());
}


#[test]
fn corners_returns_an_array_with_1_3_6_9_on_3x3_board() {
    let test_board = board_3x3();
    let expected = [1, 3, 6, 9];
    assert_eq!(expected, test_board.corners());
}

#[test]
fn corners_returns_an_array_with_1_4_12_16_on_3x3_board() {
    let test_board = board_4x4();
    let expected = [1, 4, 12, 16];
    assert_eq!(expected, test_board.corners());
}
