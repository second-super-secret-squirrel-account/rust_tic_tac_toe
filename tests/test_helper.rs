extern crate tic_tac_toe;
use tic_tac_toe::game_board::grid::Grid;
//use tic_tac_toe::game_board::game_board_traits::GameBoard;
use tic_tac_toe::game_io::game_screen::sprite::Sprite;
use tic_tac_toe::game_io::game_screen::sprite::point::Point;
use tic_tac_toe::game_io::game_screen::sprite::color::Color;
use tic_tac_toe::game_io::game_screen::sprite::sprite_data::SpriteData;

#[allow(dead_code)]
pub const FILL: usize = 999;

#[allow(dead_code)]
pub const TEST_MOVE: usize = 1;

#[allow(dead_code)]
pub const TEST_PLAYER_1: usize = 10;

#[allow(dead_code)]
pub const TEST_PLAYER_2: usize = 20;

#[allow(dead_code)]
pub fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

#[allow(dead_code)]
pub fn test_sprite_data() -> SpriteData {
    let mut this = SpriteData::new();
    this.add_line("test_sprite".to_string());
    this.add_line("line_2".to_string());
    this
}
#[allow(dead_code)]
pub fn test_sprite() -> Sprite {
    Sprite::new(point(0, 0), Color::WhiteOnBlue, test_sprite_data())
}

#[allow(dead_code)]
pub fn board_3x3() -> Grid<usize> {
    Grid::new(3, 3, FILL)
}
#[allow(dead_code)]
pub fn board_4x4() -> Grid<usize> {
    Grid::new(4, 4, FILL)
}
