extern crate tic_tac_toe;
use tic_tac_toe::sprite::point::Point;


#[test]
fn test_constructor_sets_x_and_y_correctly() {
    let test_point = Point::new(3, 4);

    assert_eq!(3, test_point.x);
    assert_eq!(4, test_point.y);
}

#[test]
fn test_that_point_works_with_floats() {
    let test_point = Point::new(3.0, 4.0);

    assert_eq!(3.0, test_point.x);
    assert_eq!(4.0, test_point.y);
}

#[test]
fn test_changing_value_of_x_and_y() {
    let mut test_point = Point::new(5, 5);

    test_point.x = test_point.x + 1;
    test_point.y = test_point.y + 2;

    assert_eq!(6, test_point.x);
    assert_eq!(7, test_point.y);

}
