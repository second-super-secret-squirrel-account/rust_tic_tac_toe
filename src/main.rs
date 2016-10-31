extern crate tic_tac_toe;
use tic_tac_toe::screen_builder::ScreenBuilder;
use tic_tac_toe::game_io;
use tic_tac_toe::game_io::key_board_input::*;
use tic_tac_toe::game_io::screen_width;
use tic_tac_toe::game_io::screen_height;
use tic_tac_toe::game_board::grid::Grid;
use tic_tac_toe::game_board::board_token::BoardToken;


use tic_tac_toe::game_rules::move_rules_traits::HasMoveRules;

use tic_tac_toe::game_board::game_board_traits::GameBoard;

use tic_tac_toe::game_rules::game_status_traits::HasGameStatus;
use tic_tac_toe::game_rules::game_state::GameState;

use tic_tac_toe::mini_max::MiniMax;



fn change_players(player: BoardToken) -> BoardToken {
    if player == BoardToken::Player1 {
        return BoardToken::Player2;
    } else {
        return BoardToken::Player1;
    }
}


fn main() {

    //setup
    game_io::start_io();


    let mut board = Grid::new(3, 3, BoardToken::Blank);
    let mut active = true;
    while active {
        let screen_builder = ScreenBuilder::new(screen_height(), screen_width());
        let mut player = BoardToken::Player1;

        //loop
        while board.game_status() == GameState::Playing {
            // draw screen
            let screen = screen_builder.playing_screen(board.clone());
            screen.draw_screen();
            if player == BoardToken::Player1 {
                let key = get_key() - 48;
                let key = key as usize;
                if key < 9 {
                    if board.valid_move(key) {
                        board.set_space(key, player.clone());
                        player = change_players(player);
                    }
                }
            } else {
                let cpu_move = MiniMax::get_move(board.clone(), player.clone(), change_players(player.clone()));
                board.set_space(cpu_move, player.clone());
                player = change_players(player);
                }
            }


            let screen = screen_builder.playing_screen(board.clone());
            screen.draw_screen();
            let key = get_key();
            if key == KEY_R {
                board = Grid::new(3, 3, BoardToken::Blank);
            } else if key == KEY_Q {
                active = false;
            }
        }
    // end
    game_io::end_io();
}
