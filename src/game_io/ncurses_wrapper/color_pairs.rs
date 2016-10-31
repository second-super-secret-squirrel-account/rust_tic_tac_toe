extern crate ncurses;

pub const WHITE_ON_BLUE: i16 = 1;
pub const RED_ON_BLUE: i16 = 2;
pub const GREEN_ON_BLUE: i16 = 3;
pub const YELLOW_ON_BLUE: i16 = 4;
pub const RED_ON_YELLOW: i16 = 5;
pub const GREEN_ON_YELLOW: i16 = 6;
pub const BLUE_ON_YELLOW: i16 = 7;


pub fn set_colors(color_pair: i16) {
    ncurses::attron(ncurses::COLOR_PAIR(color_pair));
}
