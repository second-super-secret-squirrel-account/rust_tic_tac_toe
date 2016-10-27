extern crate tic_tac_toe;
mod test_helper;
use test_helper::*;
use tic_tac_toe::game_board::basic_board_traits::BasicBoard;

#[test]
fn board_sets_blank_value() {
    let test_board = board_3x3();

    assert_eq!(*test_board.blank_value(), FILL);
}

// Testing a 3 by 3 Game Board
#[test]
fn new_board_is_filled_with_blank_board_tokens() {
    let test_board = board_3x3();

    for i in 1..10 {
        assert_eq!(*test_board.get_space(i), FILL);
    }
}

#[test]
fn setting_board_spaces() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_MOVE);

    assert_eq!(*test_board.get_space(1), TEST_MOVE);
}

#[test]
fn trying_to_set_space_zero_does_not_crash() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_MOVE);
}

#[test]
fn trying_to_set_space_above_9_on_3by3_board_does_not_crash() {
    let mut test_board = board_3x3();

    test_board.set_space(10, TEST_MOVE);
}

// Testing a 4 x 4 Game Board
#[test]
fn a_4by4_board_can_set_space_10() {
    let mut test_board = board_4x4();

    test_board.set_space(10, TEST_MOVE);

    assert_eq!(*test_board.get_space(10), TEST_MOVE);
}

#[test]
fn trying_to_set_space_above_16_on_4by4_board_does_not_crash() {
    let mut test_board = board_4x4();

    test_board.set_space(17, TEST_MOVE);
}
