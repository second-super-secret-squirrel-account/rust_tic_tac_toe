extern crate ncurses;

pub fn print<S: Into<String>>(x: i32, y: i32, value: S) {
    ncurses::mvprintw(x, y, (format!("{}", value.into()).as_ref()));
}

pub fn clear() {
    ncurses::clear();
}

pub fn refresh() {
    ncurses::refresh();
}
