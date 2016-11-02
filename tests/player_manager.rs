extern crate tic_tac_toe;

use tic_tac_toe::player_manager::PlayerManager;
use tic_tac_toe::player_manager::player::PlayerType;
use tic_tac_toe::player_manager::player::Player;
use tic_tac_toe::game_board::board_token::BoardToken;

#[test]
fn change_players_swaps_players() {
    let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
    let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
    let mut player_manager = PlayerManager::new(player1.clone(), player2.clone());

    player_manager.change_turn();

    assert_eq!(player2, *player_manager.current_player());
}

#[test]
fn current_player_returns_player_1() {
    let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
    let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
    let player_manager = PlayerManager::new(player1.clone(), player2.clone());

    assert_eq!(player1, *player_manager.current_player());
}

#[test]
fn non_current_player_returns_player_2() {
    let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
    let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
    let player_manager = PlayerManager::new(player1.clone(), player2.clone());

    assert_eq!(player2, *player_manager.non_current_player());
}
