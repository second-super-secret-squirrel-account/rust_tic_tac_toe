extern crate tic_tac_toe;
mod test_helper;
use test_helper::*;
use tic_tac_toe::game_board::game_board_traits::GameBoard;
use tic_tac_toe::game_rules::move_rules_traits::*;

#[test]
fn valid_move_returns_ture_if_space_is_blank_token() {
    let test_board = board_3x3();

    assert!(test_board.valid_move(TEST_MOVE));
}

#[test]
fn valid_move_returns_false_if_space_is_not_blank_token() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_MOVE);

    assert_eq!(false, test_board.valid_move(TEST_MOVE));
}


#[test]
fn available_moves_count_returns_9_on_blank_3x3_board() {
    let test_board = board_3x3();

    assert_eq!(9, test_board.available_moves_count());
}

#[test]
fn available_moves_count_returns_8_after_1_move_on_3x3_board() {
    let mut test_board = board_3x3();

    test_board.set_space(TEST_MOVE, TEST_MOVE);

    assert_eq!(8, test_board.available_moves_count());
}

#[test]
fn available_moves_count_returns_0_after_9_moves_on_3x3_board() {
    let mut test_board = board_3x3();

    for index in 0..9 {
        test_board.set_space(index, TEST_MOVE);
    }

    assert_eq!(0, test_board.available_moves_count());
}

#[test]
fn available_moves_returns_vec_of_1_to_9_on_empty_board() {
    let test_board = board_3x3();

    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(expected, test_board.available_moves());
}

#[test]
fn available_moves_returns_vec_does_not_have_1_on_board_where_1_is_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_MOVE);

    let expected = vec![0, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(expected, test_board.available_moves());
}

#[test]
fn available_moves_returns_vec_of_1_to_16_on_empty_board_4x4() {
    let test_board = board_4x4();

    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert_eq!(expected, test_board.available_moves());
}

#[test]
fn available_moves_returns_vec_does_not_have_1_on_board_4x4_where_1_is_not_blank() {
    let mut test_board = board_4x4();

    test_board.set_space(1, TEST_MOVE);

    let expected = vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert_eq!(expected, test_board.available_moves());
}
