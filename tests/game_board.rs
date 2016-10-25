extern crate tic_tac_toe;
use tic_tac_toe::game_board::Board;
use tic_tac_toe::board_token::BoardToken;

// Testing a 3 by 3 Game Board
#[test]
fn test_new_board_is_filled_with_blank_board_tokens() {
    let test_board = Board::new(9);
    for i in 1..10 {
        assert_eq!(*test_board.get_space(i), BoardToken::BLANK);
    }
}

#[test]
fn test_setting_board_spaces() {
    let mut test_board = Board::new(9);
    test_board.set_space(1, BoardToken::PLAYER1);
    assert_eq!(*test_board.get_space(1), BoardToken::PLAYER1);
}

#[test]
fn test_that_trying_to_set_space_zero_does_not_crash() {
    let mut test_board = Board::new(9);
    test_board.set_space(0, BoardToken::PLAYER1);
}

#[test]
fn test_that_trying_to_set_space_above_9_on_3by3_board_does_not_crash() {
    let mut test_board = Board::new(9);
    test_board.set_space(10, BoardToken::PLAYER1);
}

// Testing a 4 x 4 Game Board
#[test]
fn test_that_a_4by4_board_can_set_space_10() {
    let mut test_board = Board::new(16);
    test_board.set_space(10, BoardToken::PLAYER1);
    assert_eq!(*test_board.get_space(10), BoardToken::PLAYER1);
}

#[test]
fn test_that_trying_to_set_space_above_16_on_4by4_board_does_not_crash() {
    let mut test_board = Board::new(16);
    test_board.set_space(17, BoardToken::PLAYER1);
}



