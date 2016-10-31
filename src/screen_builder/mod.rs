pub mod sprite_sheet;
pub mod point_generator;

use game_board::grid::Grid;
use game_board::board_token::BoardToken;
use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;
use game_io::game_screen::sprite::sprite_data_traits::Drawable;

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

    pub fn playing_screen(&self, board: Grid<BoardToken>) -> GameScreen {
        let mut screen = GameScreen::new();
        screen.add_sprite(self.title_bar_sprite());
        screen.add_sprite(self.board_sprite());


        let board_width = sprite_sheet::board().width();
        let board_height = sprite_sheet::board().height();

        let board_point = point_generator::center(board_height,
                                                  board_width,
                                                  self.screen_height,
                                                  self.screen_width);

        screen.add_sprite(Sprite::new(board_point.clone(), Color::WhiteOnBlue, sprite_sheet::board()));



        for row in 0..3 {
            for col in 0..3 {
                let x = (row * 8) + 2 + board_point.clone().x();
                let y = (col * 11) + 2 + board_point.clone().y();
                let row_usize = row as usize;
                let col_uszie = col as usize;
                let point = Point::new(x , y);
                let sprite_data = board.clone().get(row_usize, col_uszie).to_sprite_data();
                screen.add_sprite(Sprite::new(point, Color::WhiteOnBlue, sprite_data));
            }
        }
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
