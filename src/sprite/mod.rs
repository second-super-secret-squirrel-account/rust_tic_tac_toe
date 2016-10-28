pub mod point;
pub mod color;
pub mod sprite_data;
use sprite::color::Color;
use sprite::point::Point;
use sprite::sprite_data::SpriteData;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sprite<T> {
    point: Point<T>,
    fg_color: Color,
    bg_color: Color,
    value: SpriteData,
}

impl <T: Clone + PartialEq> Sprite<T> {
    pub fn new(point: Point<T>,
               fg_color: Color,
               bg_color: Color,
               value: SpriteData
               ) -> Sprite<T> {
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

    pub fn point(&self) -> &Point<T> {
        &self.point
    }

    pub fn fg_color(&self) -> &Color {
        &self.fg_color
    }

    pub fn bg_color(&self) -> &Color {
        &self.bg_color
    }
}
