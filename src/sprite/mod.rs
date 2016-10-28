pub mod point;
pub mod color;
use sprite::color::Color;
use sprite::point::Point;

pub struct Sprite<T> {
    pub point: Point<T>,
    pub fg_color: Color,
    pub bg_color: Color,
    pub value: Vec<String>,
}

impl <T: Clone + PartialEq> Sprite<T> {
    pub fn new(point: Point<T>,
               fg_color: Color,
               bg_color: Color,
               value: Vec<String>
               ) -> Sprite<T> {
        Sprite {
            point: point,
            fg_color: fg_color,
            bg_color: bg_color,
            value: value,
        }
    }
}
