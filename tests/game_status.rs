extern crate tic_tac_toe;
mod test_helper;
use test_helper::*;
use tic_tac_toe::game_board::game_board_traits::GameBoard;
use tic_tac_toe::game_rules::game_status_traits::*;
use tic_tac_toe::game_rules::game_state::GameState;


#[test]
fn game_status_returns_game_state_playing_on_empty_board() {
    let test_board = board_3x3();

    let expected = GameState::Playing;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_tied_when_no_winner_and_board_is_full() {
    let mut test_board = board_3x3();

    for num in 0..9 {
        test_board.set_space(num, num);
        println!("set_space {} ", num);
    }

    let expected = GameState::Tied;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_first_row_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    for index in 0..3 {
        test_board.set_space(index, TEST_MOVE);
    }

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_second_row_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    for index in 3..6 {
        test_board.set_space(index, TEST_MOVE);
    }

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_third_row_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    for index in 6..9 {
        test_board.set_space(index, TEST_MOVE);
    }

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_diagnol_ascending_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(2, TEST_MOVE);
    test_board.set_space(4, TEST_MOVE);
    test_board.set_space(6, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_diagnol_decending_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_MOVE);
    test_board.set_space(4, TEST_MOVE);
    test_board.set_space(8, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_fist_col_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_MOVE);
    test_board.set_space(3, TEST_MOVE);
    test_board.set_space(6, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_second_col_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_MOVE);
    test_board.set_space(4, TEST_MOVE);
    test_board.set_space(7, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_third_col_is_all_the_same_and_not_blank() {
    let mut test_board = board_3x3();

    test_board.set_space(2, TEST_MOVE);
    test_board.set_space(5, TEST_MOVE);
    test_board.set_space(8, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_status_returns_game_state_winner_when_first_col_is_all_the_same_for_4x4() {
    let mut test_board = board_4x4();

    test_board.set_space(0, TEST_MOVE);
    test_board.set_space(4, TEST_MOVE);
    test_board.set_space(8, TEST_MOVE);
    test_board.set_space(12, TEST_MOVE);

    let expected = GameState::Winner;
    assert_eq!(expected, test_board.game_status());
}

#[test]
fn game_winner_returns_1_when_that_is_winner() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_MOVE);
    test_board.set_space(1, TEST_MOVE);
    test_board.set_space(2, TEST_MOVE);

    assert_eq!(1, test_board.game_winner());
}

#[test]
fn game_winner_returns_fill_value_when_that_is_no_winner() {
    let mut test_board = board_3x3();

    assert_eq!(FILL, test_board.game_winner());
}
