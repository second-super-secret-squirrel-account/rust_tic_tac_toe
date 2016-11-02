extern crate tic_tac_toe;

use tic_tac_toe::game_io::screen_builder::playing_screen::sprite_sheet::winner;
use tic_tac_toe::game_board::board_token::BoardToken;

#[test]
fn winner_returns_x_is_the_winner_when_x_wins() {
    let test_sprite_data = winner(BoardToken::PlayerX);

    assert_eq!("X is the Winner", test_sprite_data.line(0));
}

#[test]
fn winner_returns_o_is_the_winner_when_o_wins() {
    let test_sprite_data = winner(BoardToken::PlayerO);

    assert_eq!("O is the Winner", test_sprite_data.line(0));
}

#[test]
fn winner_returns_tied_is_the_winner_when_blank_wins() {
    let test_sprite_data = winner(BoardToken::Blank);

    assert_eq!("Tied", test_sprite_data.line(0));
}
