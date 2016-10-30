pub mod point;
pub mod color;
pub mod sprite_data;
pub mod sprite_data_traits;
use sprite::color::Color;
use sprite::point::Point;
use sprite::sprite_data::SpriteData;

pub struct Sprite {
    point: Point,
    color: Color,
    value: SpriteData,
}

impl Sprite {
    pub fn new(point: Point,
               color: Color,
               value: SpriteData
               ) -> Sprite {
        Sprite {
            point: point,
            color: color,
            value: value,
        }
    }

    pub fn value(&self) -> &SpriteData {
        &self.value
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}
