extern crate tic_tac_toe;
use tic_tac_toe::game_io;
use tic_tac_toe::tic_tac_toe_game::setup::TicTacToeGameSetup;

fn main() {

    //setup
    game_io::start_io();


    let mut setup = TicTacToeGameSetup::new();
    setup.run();

    game_io::end_io();

}
