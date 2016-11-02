extern crate tic_tac_toe;
use tic_tac_toe::game_io::game_screen::sprite::Sprite;
use tic_tac_toe::game_io::game_screen::sprite::point::Point;
use tic_tac_toe::game_io::game_screen::sprite::color::Color;
use tic_tac_toe::game_io::game_screen::sprite::sprite_data::SpriteData;

#[test]
fn test_constructor_properly_sets_point_colors_and_value() {
    let point = Point::new(2, 4);
    let color = Color::WhiteOnBlue;

    let data = SpriteData::new();
    let sprite = Sprite::new(point.clone(), color.clone(), data.clone());

    assert_eq!(point, *sprite.point());
    assert_eq!(color, *sprite.color());
    assert_eq!(data, *sprite.data());
}
