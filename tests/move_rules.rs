extern crate tic_tac_toe;
use tic_tac_toe::game_board::Board;
use tic_tac_toe::board_token::BoardToken;
use tic_tac_toe::move_rules;

#[test]
fn valid_move_returns_ture_if_space_is_blank_token() {
    let test_board = Board::new(9);
    assert!(move_rules::valid_move(test_board, 1));
}

#[test]
fn valid_move_returns_false_if_space_is_not_blank_token() {
    let mut test_board = Board::new(9);
    test_board.set_space(1, BoardToken::Player1);
    assert_eq!(false, move_rules::valid_move(test_board, 1));
}

#[test]
fn moves_remaining_returns_9_on_blank_3x3_board() {
    let test_board = Board::new(9);
    assert_eq!(9, move_rules::moves_remaining(test_board));
}

#[test]
fn moves_remaining_returns_8_on_blank_3x3_board_after_one_move_played() {
    let mut test_board = Board::new(9);
    test_board.set_space(1, BoardToken::Player1);
    assert_eq!(8, move_rules::moves_remaining(test_board));
}

#[test]
fn moves_remaining_returns_0_on_blank_3x3_board_after_nine_move_played() {
    let mut test_board = Board::new(9);
    for index in 0..10 {
        test_board.set_space(index, BoardToken::Player1);
    }
    assert_eq!(0, move_rules::moves_remaining(test_board));
}

#[test]
fn available_spaces_returns_vec_of_1_to_9_on_empty_board() {
    let test_board = Board::new(9);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(expected, move_rules::available_spaces(test_board));
}


#[test]
fn available_spaces_returns_vec_of_2_to_9_on_board_where_1_is_not_blank() {
    let mut test_board = Board::new(9);
    test_board.set_space(1, BoardToken::Player1);
    let expected = vec![2, 3, 4, 5, 6, 7, 8, 9];

    assert_eq!(expected, move_rules::available_spaces(test_board));
}

#[test]
fn available_spaces_returns_vec_of_1_to_16_on_empty_4x4_board() {
    let test_board = Board::new(16);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert_eq!(expected, move_rules::available_spaces(test_board));
}


#[test]
fn available_spaces_returns_vec_of_2_to_16_on_4x4_board_where_1_is_not_on_blank() {
    let mut test_board = Board::new(16);
    test_board.set_space(1, BoardToken::Player1);
    let expected = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    assert_eq!(expected, move_rules::available_spaces(test_board));
}
