pub mod playing_screen;
pub mod point_generator;
pub mod sprite_sheet;

use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;


pub struct ScreenBuilder {
    screen_width: i32,
    screen_height: i32,
}

impl ScreenBuilder {
    pub fn new(screen_height: i32, screen_width: i32) -> ScreenBuilder {
        ScreenBuilder {
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }

    fn background_sprite(&self) -> Sprite {
        let point = Point::new(0, 0);
        let data = sprite_sheet::background(self.screen_height, self.screen_width);
        let color = Color::WhiteOnBlue;

        Sprite::new(point, color, data)
    }

    fn title_bar_sprite(&self) -> Sprite {
        let sprite_data = sprite_sheet::title();
        let sprite_width = sprite_sheet::title().width();
        let sprite_y = point_generator::center_side(sprite_width, self.screen_width);
        let point = Point::new(5, sprite_y);
        Sprite::new(point, Color::WhiteOnBlue, sprite_data)
    }
}
