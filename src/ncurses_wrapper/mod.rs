extern crate ncurses;

pub fn init() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn end() {
    ncurses::endwin();
}

pub fn print<S: Into<String>>(x: i32, y: i32, value: S) {
    ncurses::mvprintw(x, y, (format!("{}", value.into()).as_ref()));
}

pub fn get_key() -> i32 {
    ncurses::getch()
}

pub fn clear() {
    ncurses::clear();
}

pub fn refresh() {
    ncurses::refresh();
}
