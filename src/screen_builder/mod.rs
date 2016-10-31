pub mod sprite_sheet;
pub mod point_generator;

use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
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

    pub fn playing_screen(&self) -> GameScreen {
        let mut screen = GameScreen::new();
        screen.add_sprite(self.title_bar_sprite());
        screen.add_sprite(self.board_sprite());
        screen
    }

    fn title_bar_sprite(&self) -> Sprite {
        let sprite_data = sprite_sheet::title();
        let sprite_width = sprite_sheet::title().width();

        let sprite_y = point_generator::center_side(sprite_width, self.screen_width);
        let point = Point::new(5, sprite_y);

        Sprite::new(point, Color::WhiteOnBlue, sprite_data)
    }

    fn board_sprite(&self) -> Sprite {
        let board_width = sprite_sheet::board().width();
        let board_height = sprite_sheet::board().height();

        let point = point_generator::center(board_height,
                                                  board_width,
                                                  self.screen_height,
                                                  self.screen_width);

        Sprite::new(point, Color::WhiteOnBlue, sprite_sheet::board())
    }
}
