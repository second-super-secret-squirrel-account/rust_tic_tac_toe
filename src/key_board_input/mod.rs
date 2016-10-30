use ncurses_wrapper;

pub const KEY_UP: i32 = ncurses_wrapper::KEY_UP;
pub const KEY_DOWN: i32 = ncurses_wrapper::KEY_DOWN;
pub const KEY_RIGHT: i32 = ncurses_wrapper::KEY_RIGHT;
pub const KEY_LEFT: i32 = ncurses_wrapper::KEY_LEFT;

pub const KEY_J: i32 = 106;
pub const KEY_K: i32 = 107;
pub const KEY_L: i32 = 108;
pub const KEY_H: i32 = 104;

pub const KEY_W: i32 = 119;
pub const KEY_A: i32 = 97;
pub const KEY_S: i32 = 115;
pub const KEY_D: i32 = 100;

pub const KEY_Q: i32 = 113;
pub const KEY_R: i32 = 114;

pub const KEY_ENTER: i32 = 10;
pub const KEY_SPACEBAR: i32 = 32;

pub fn get_key() -> i32 {
    ncurses_wrapper::get_key()
}

pub fn is_up(key: i32) -> bool {
    key == KEY_UP || key == KEY_W || key == KEY_K
}

pub fn is_left(key: i32) -> bool {
    key == KEY_LEFT || key == KEY_A || key == KEY_H
}

pub fn is_down(key: i32) -> bool {
    key == KEY_DOWN || key == KEY_S || key == KEY_J
}

pub fn is_right(key: i32) -> bool {
    key == KEY_RIGHT || key == KEY_D || key == KEY_L
}

pub fn is_select(key: i32) -> bool {
    key == KEY_ENTER || key == KEY_SPACEBAR
}
