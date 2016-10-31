pub mod ncurses_wrapper;
pub mod game_screen;
pub mod screen_builder;
pub mod key_board_input;

pub fn start_io() {
    ncurses_wrapper::init()
}

pub fn end_io() {
    ncurses_wrapper::end();
}

pub fn screen_width() -> i32{
    ncurses_wrapper::get_width()
}

pub fn screen_height() -> i32 {
    ncurses_wrapper::get_height()
}
