extern crate tic_tac_toe;
use tic_tac_toe::grid::Grid;
use tic_tac_toe::sprite::Sprite;
use tic_tac_toe::sprite::point::Point;
use tic_tac_toe::sprite::color::Color;

#[allow(dead_code)]
pub const FILL: usize = 999;

#[allow(dead_code)]
pub const TEST_MOVE: usize = 1;

#[allow(dead_code)]
pub fn point(x: i32, y: i32) -> Point<i32> {
    Point::new(x, y)
}
#[allow(dead_code)]
pub fn test_vec_of_strings() -> Vec<String> {
    let mut this = Vec::new();
    this.push("test_sprite".to_string());
    this.push("line_2".to_string());
    this
}
#[allow(dead_code)]
pub fn test_sprite() -> Sprite<i32> {
    Sprite::new(point(0, 0), Color::White, Color::Black, test_vec_of_strings())
}

#[allow(dead_code)]
pub fn board_3x3() -> Grid<usize> {
    Grid::new(3, 3, FILL)
}
#[allow(dead_code)]
pub fn board_4x4() -> Grid<usize> {
    Grid::new(4, 4, FILL)
}
