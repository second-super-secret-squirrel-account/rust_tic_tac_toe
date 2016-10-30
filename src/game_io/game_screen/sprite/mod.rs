pub mod point;
pub mod color;
pub mod sprite_data;
pub mod sprite_data_traits;
use game_io::game_screen::sprite::color::Color;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::sprite_data::SpriteData;


use game_io::ncurses_wrapper::output::*;
use game_io::ncurses_wrapper::color_pairs::set_colors;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sprite {
    point: Point,
    color: Color,
    data: SpriteData,
}

impl Sprite {
    pub fn new(point: Point,
               color: Color,
               data: SpriteData
               ) -> Sprite {
        Sprite {
            point: point,
            color: color,
            data: data,
        }
    }

    pub fn data(&self) -> &SpriteData {
        &self.data
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn draw(&self) {
        set_colors(self.color.value());
        for index in 0..self.data.len() {
            let offset = index as i32;
            print((self.point.x() + offset), self.point.y(), self.data.line(index));
        }
    }
}
