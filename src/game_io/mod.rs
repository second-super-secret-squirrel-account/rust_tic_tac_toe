pub mod ncurses_wrapper;
pub mod game_screen;
pub mod key_board_input;

pub fn start_io() {
    ncurses_wrapper::init()
}

pub fn end_io() {
    ncurses_wrapper::end();
}
