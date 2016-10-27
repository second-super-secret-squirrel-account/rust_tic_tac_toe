extern crate tic_tac_toe;
use tic_tac_toe::board_token::BoardToken;
use tic_tac_toe::game_text::GameText;

#[test]
fn test() {
    let test_text = GameText::new('a', 'b');

    test_text.winner_string(BoardToken::Player1);
    assert!(false);
}

