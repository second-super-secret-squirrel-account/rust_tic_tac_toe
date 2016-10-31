pub mod sprite_sheet;
pub mod point_generator;

use game_board::grid::Grid;
use game_board::board_token::BoardToken;
use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;
use game_io::game_screen::sprite::sprite_data_traits::Drawable;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::game_state::GameState;

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
        screen.add_sprite(self.background_sprite());
        screen.add_sprite(self.title_bar_sprite());
        screen.add_sprite(self.board_sprite());

        for sprite in self.token_spites(board.clone()) {
            screen.add_sprite(sprite);
        }

        if board.game_status() != GameState::Playing {
            screen.add_sprite(self.bottom_menu_sprite());

        }

        screen
    }

    fn bottom_menu_sprite(&self) -> Sprite {
        let point = Point::new (self.screen_height - 3, 3);
        let color = Color::YellowOnBlue;
        let data = sprite_sheet::bottom_menu();

        Sprite::new(point, color, data)

    }

    fn background_sprite(&self) -> Sprite {
        let point = Point::new(0, 0);
        let data = sprite_sheet::background(self.screen_height, self.screen_width);
        let color = Color::WhiteOnBlue;

        Sprite::new(point, color, data)


    }

    fn token_spites(&self, board: Grid<BoardToken>) -> Vec<Sprite> {
        let mut tokens = Vec::new();
        let board_width = sprite_sheet::board().width();
        let board_height = sprite_sheet::board().height();
        let off_set_height = point_generator::center_side(board_height, self.screen_height);
        let off_set_width = point_generator::center_side(board_width, self.screen_width);
        for row in 0..3 {
            for col in 0..3 {
                let x = (row * 8) + 2 + off_set_height;
                let y = (col * 11) + 2 + off_set_width;
                let row_usize = row as usize;
                let col_uszie = col as usize;
                let point = Point::new(x , y);
                let sprite_data = board.clone().get(row_usize, col_uszie).to_sprite_data();
                tokens.push(Sprite::new(point, Color::WhiteOnBlue, sprite_data));
            }
        }
        tokens
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
