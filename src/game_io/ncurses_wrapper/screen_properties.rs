extern crate ncurses;

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
