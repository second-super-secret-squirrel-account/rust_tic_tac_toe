extern crate ncurses;

pub mod color_pairs;
pub mod input;
pub mod output;


use game_io::ncurses_wrapper::color_pairs::*;


pub fn init() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::start_color();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    init_color_pairs();
}

pub fn end() {
    ncurses::endwin();
}

fn init_color_pairs() {
    ncurses::init_pair(WHITE_ON_BLUE, ncurses::COLOR_WHITE, ncurses::COLOR_BLUE);
    ncurses::init_pair(RED_ON_BLUE, ncurses::COLOR_RED, ncurses::COLOR_BLUE);
    ncurses::init_pair(GREEN_ON_BLUE, ncurses::COLOR_BLUE, ncurses::COLOR_BLUE);
    ncurses::init_pair(YELLOW_ON_BLUE, ncurses::COLOR_YELLOW, ncurses::COLOR_BLUE);
    ncurses::init_pair(RED_ON_YELLOW, ncurses::COLOR_RED, ncurses::COLOR_YELLOW);
    ncurses::init_pair(GREEN_ON_YELLOW, ncurses::COLOR_GREEN, ncurses::COLOR_YELLOW);
    ncurses::init_pair(BLUE_ON_YELLOW, ncurses::COLOR_BLUE, ncurses::COLOR_YELLOW);
}

pub fn get_width() -> i32 {
    let mut width = 0;
    let mut height = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut height, &mut width);
    width
}

pub fn get_height() -> i32 {
    let mut width = 0;
    let mut height = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut height, &mut width);
    height
}
