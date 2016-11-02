extern crate tic_tac_toe;

mod test_helper;
use test_helper::*;
use tic_tac_toe::game_board::game_board_traits::GameBoard;


use tic_tac_toe::player_manager::mini_max::MiniMax;

#[test]
fn ai_goes_for_the_block() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_PLAYER_1);
    test_board.set_space(1, TEST_PLAYER_1);

    let chosen_move = MiniMax::get_move(test_board, TEST_PLAYER_2, TEST_PLAYER_1);
    assert_eq!(2, chosen_move)
}

#[test]
fn ai_goes_for_the_win_over_a_block() {
    let mut test_board = board_3x3();

    test_board.set_space(1, TEST_PLAYER_1);
    test_board.set_space(3, TEST_PLAYER_2);
    test_board.set_space(4, TEST_PLAYER_1);
    test_board.set_space(6, TEST_PLAYER_1);
    test_board.set_space(2, TEST_PLAYER_2);
    test_board.set_space(5, TEST_PLAYER_2);
    test_board.set_space(7, TEST_PLAYER_2);

    let chosen_move = MiniMax::get_move(test_board, TEST_PLAYER_2, TEST_PLAYER_1);
    assert_eq!(8, chosen_move);
}

#[test]
fn ai_will_play_move_that_creates_fork() {
    let mut test_board = board_3x3();

    test_board.set_space(3, TEST_PLAYER_1);
    test_board.set_space(8, TEST_PLAYER_1);
    test_board.set_space(0, TEST_PLAYER_2);
    test_board.set_space(6, TEST_PLAYER_2);

    let chosen_move = MiniMax::get_move(test_board, TEST_PLAYER_2, TEST_PLAYER_1);
    assert_eq!(2, chosen_move);
}

#[test]
fn ai_will_play_move_that_prevents_opponent_from_creating_fork() {
    let mut test_board = board_3x3();

    test_board.set_space(0, TEST_PLAYER_1);
    test_board.set_space(8, TEST_PLAYER_1);
    test_board.set_space(4, TEST_PLAYER_2);

    let chosen_move = MiniMax::get_move(test_board, TEST_PLAYER_2, TEST_PLAYER_1);
    assert_eq!(1, chosen_move);
}
