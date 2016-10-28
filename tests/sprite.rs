extern crate tic_tac_toe;
use tic_tac_toe::sprite::Sprite;
use tic_tac_toe::sprite::point::Point;
use tic_tac_toe::sprite::color::Color;
#[test]
fn test_constructor_properly_sets_point_colors_and_value() {
    let point = Point::new(2, 4);
    let fg_color = Color::Red;
    let bg_color = Color::Black;
    let value = vec!["hello".to_string(); 3];
    let sprite = Sprite::new(point.clone(), fg_color.clone(), bg_color.clone(), value.clone());

    assert_eq!(point, sprite.point);
    assert_eq!(fg_color, sprite.fg_color);
    assert_eq!(bg_color, sprite.bg_color);
    assert_eq!(value, sprite.value);
}
