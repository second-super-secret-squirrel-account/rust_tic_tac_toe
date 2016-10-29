extern crate tic_tac_toe;

use tic_tac_toe::board_token::BoardToken;

#[test]
fn test_board_token_to_string_gets_x_for_player_1() {
    let token = BoardToken::Player1;

    assert_eq!("X", token.to_string());
}

#[test]
fn test_board_token_to_string_gets_o_for_player_2() {
    let token = BoardToken::Player2;

    assert_eq!("O", token.to_string());
}

#[test]
fn test_board_token_to_string_gets_space_for_blank() {
    let token = BoardToken::Blank;

    assert_eq!(" ", token.to_string());
}
