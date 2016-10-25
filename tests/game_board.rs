extern crate tic_tac_toe;
use tic_tac_toe::game_board::Board;
use tic_tac_toe::board_token::BoardToken;

// Testing a 3 by 3 Game Board
#[test]
fn new_board_is_filled_with_blank_board_tokens() {
    let test_board = Board::new(9);
    for i in 1..10 {
        assert_eq!(*test_board.get_space(i), BoardToken::Blank);
    }
}

#[test]
fn setting_board_spaces() {
    let mut test_board = Board::new(9);
    test_board.set_space(1, BoardToken::Player1);
    assert_eq!(*test_board.get_space(1), BoardToken::Player1);
}

#[test]
fn trying_to_set_space_zero_does_not_crash() {
    let mut test_board = Board::new(9);
    test_board.set_space(0, BoardToken::Player1);
}

#[test]
fn trying_to_set_space_above_9_on_3by3_board_does_not_crash() {
    let mut test_board = Board::new(9);
    test_board.set_space(10, BoardToken::Player1);
}

// Testing a 4 x 4 Game Board
#[test]
fn a_4by4_board_can_set_space_10() {
    let mut test_board = Board::new(16);
    test_board.set_space(10, BoardToken::Player1);
    assert_eq!(*test_board.get_space(10), BoardToken::Player1);
}

#[test]
fn trying_to_set_space_above_16_on_4by4_board_does_not_crash() {
    let mut test_board = Board::new(16);
    test_board.set_space(17, BoardToken::Player1);
}

#[test]
fn size_return_9_for3x3_board() {
    let test_board = Board::new(9);
    assert_eq!(9, test_board.size());
}

#[test]
fn corners_returns_an_array_with_1_3_6_9_on_a_3x3_board() {
    let test_board = Board::new(9);
    let expected = [1, 3, 6, 9];
    assert_eq!(expected, test_board.corners());
}

#[test]
fn corners_returns_an_array_with_1_4_12_16_on_a_4x4_board() {
    let test_board = Board::new(16);
    let expected = [1, 4, 12, 16];
    assert_eq!(expected, test_board.corners());
}

#[test]
fn row_size_returns_3_on_3x3_board() {
    let test_board = Board::new(9);
    assert_eq!(3, test_board.row_size());
}

