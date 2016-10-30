extern crate tic_tac_toe;
use tic_tac_toe::sprite::Sprite;
use tic_tac_toe::sprite::point::Point;
use tic_tac_toe::sprite::color::Color;
use tic_tac_toe::sprite::sprite_data::SpriteData;

#[test]
fn test_constructor_properly_sets_point_colors_and_value() {
    let point = Point::new(2, 4);
    let color = Color::WhiteOnBlue;

    let value = SpriteData::new();
    let sprite = Sprite::new(point.clone(), color.clone(), value.clone());

    assert_eq!(point, *sprite.point());
    assert_eq!(color, *sprite.color());
    assert_eq!(value, *sprite.value());
}
