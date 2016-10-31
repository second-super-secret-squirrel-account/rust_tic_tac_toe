extern crate tic_tac_toe;
use tic_tac_toe::game_io::game_screen::sprite::point::Point;
use tic_tac_toe::game_io::screen_builder::point_generator;

#[test]
fn center_sprite_on_screen_returns_a_point_of_25_50_for_sprite_50x100_on_100_200_screen() {
    let point = point_generator::center(50, 100, 100, 200);
    let expected = Point::new(25, 50);
    assert_eq!(expected, point);
}

#[test]
fn center_side_returns_50_for_100_200() {
    assert_eq!(50, point_generator::center_side(100, 200));
}
