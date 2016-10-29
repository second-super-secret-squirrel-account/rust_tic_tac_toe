pub mod point;
pub mod color;
pub mod sprite_data;
pub mod sprite_data_traits;
use sprite::color::Color;
use sprite::point::Point;
use sprite::sprite_data::SpriteData;

pub struct Sprite {
    point: Point,
    fg_color: Color,
    bg_color: Color,
    value: SpriteData,
}

impl Sprite {
    pub fn new(point: Point,
               fg_color: Color,
               bg_color: Color,
               value: SpriteData
               ) -> Sprite {
        Sprite {
            point: point,
            fg_color: fg_color,
            bg_color: bg_color,
            value: value,
        }
    }

    pub fn value(&self) -> &SpriteData {
        &self.value
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn fg_color(&self) -> &Color {
        &self.fg_color
    }

    pub fn bg_color(&self) -> &Color {
        &self.bg_color
    }
}
