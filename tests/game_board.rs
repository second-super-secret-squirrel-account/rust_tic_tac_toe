extern crate tic_tac_toe;
mod test_helper;
use test_helper::*;
use tic_tac_toe::game_board::game_board_traits::GameBoard;

#[test]
fn board_sets_blank_value() {
    let test_board = board_3x3();

    assert_eq!(test_board.fill_value(), FILL);
}

// Testing a 3 by 3 Game Board
#[test]
fn new_board_is_filled_with_blank_board_tokens() {
    let test_board = board_3x3();

    for i in 1..9 {
        assert_eq!(test_board.get_space(i), FILL);
    }
}

#[test]
fn setting_board_spaces_converts_col_correctly() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_MOVE);

    assert_eq!(test_board.get(0, 1), TEST_MOVE);
}

#[test]
fn setting_board_spaces_converts_row_correctly() {
    let mut test_board = board_3x3();

    test_board.set_space(3, TEST_MOVE);

    assert_eq!(test_board.get(1, 0), TEST_MOVE);
}

#[test]
fn trying_to_set_space_zero_does_not_crash() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_MOVE);
}

#[test]
fn set_space_selects_the_correct_rol_col_for_4x4() {
    let mut test_board = board_4x4();

    test_board.set_space(11, TEST_MOVE);

    assert_eq!(test_board.get(2, 3), TEST_MOVE);
}
