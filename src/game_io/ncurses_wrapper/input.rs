extern crate ncurses;

pub fn get_key() -> i32 {
    ncurses::getch()
}
pub const KEY_UP: i32 = ncurses::KEY_UP;
pub const KEY_DOWN: i32 = ncurses::KEY_DOWN;
pub const KEY_LEFT: i32 = ncurses::KEY_LEFT;
pub const KEY_RIGHT: i32 = ncurses::KEY_RIGHT;
