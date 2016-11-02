extern crate tic_tac_toe;
use tic_tac_toe::game_io;
use tic_tac_toe::tic_tac_toe_game::TicTacToeGame;
use tic_tac_toe::player_manager::player::PlayerType;
use tic_tac_toe::player_manager::player::Player;
use tic_tac_toe::player_manager::PlayerManager;
use tic_tac_toe::game_board::board_token::BoardToken;


fn main() {

    //setup
    game_io::start_io();


    let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
    let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
    let player_manager = PlayerManager::new(player1, player2);

    let mut tic_tat_toe_game = TicTacToeGame::new(player_manager);

    tic_tat_toe_game.play();

    game_io::end_io();

}
