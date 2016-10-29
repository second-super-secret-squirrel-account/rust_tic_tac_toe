extern crate tic_tac_toe;
use tic_tac_toe::sprite::point::Point;

#[test]
fn test_constructor_sets_x_and_y_correctly() {
    let test_point = Point::new(3, 4);

    assert_eq!(3, test_point.x());
    assert_eq!(4, test_point.y());
}
