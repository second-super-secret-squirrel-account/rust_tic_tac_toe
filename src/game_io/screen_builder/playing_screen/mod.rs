pub mod sprite_sheet;

use game_io::screen_builder::ScreenBuilder;
use game_io::screen_builder::point_generator;
use game_board::grid::Grid;
use game_board::board_token::BoardToken;
use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;
use game_io::game_screen::sprite::sprite_data_traits::Drawable;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::game_state::GameState;
use tic_tac_toe_game::cursor::Cursor;


impl ScreenBuilder {
    pub fn playing_screen(&self, board: Grid<BoardToken>, cursor: Cursor) -> GameScreen {
        let mut screen = self.basic_screen();
        screen.add_sprite(self.board_sprite());
        screen.add_sprite(self.cursor_sprite(cursor));

        for sprite in self.token_spites(board.clone()) {
            screen.add_sprite(sprite);
        }

        if board.game_status() != GameState::Playing {
            screen.add_sprite(self.game_over_bottom_menu_sprite());
        }
        screen
    }

    fn cursor_sprite(&self, cursor: Cursor) -> Sprite {
        let data = sprite_sheet::board_cursor();
        let x = self.cursor_x(cursor.row(),
        data.height());
        let y = self.cursor_y(cursor.col(),
        data.width());
        let point = Point::new(x, y);
        let color = Color::RedOnBlue;
        Sprite::new(point, color, data)

    }

    fn game_over_bottom_menu_sprite(&self) -> Sprite {
        let point = Point::new (self.screen_height - 3, 3);
        let color = Color::YellowOnBlue;
        let data = sprite_sheet::bottom_menu();

        Sprite::new(point, color, data)

    }

    fn cursor_x(&self, row: i32, sprite_height: i32) -> i32 {
        let board_height = sprite_sheet::board().height();
        let off_set_height = point_generator::center_side(board_height, self.screen_height);
        (row * (1 + sprite_height)) + 1 + off_set_height
    }

    fn cursor_y(&self, col: i32, sprite_width: i32) -> i32 {
        let board_width = sprite_sheet::board().width();
        let off_set_width = point_generator::center_side(board_width, self.screen_width);
        (col * (1 + sprite_width)) + 1 + off_set_width
    }

    fn board_square_x(&self, row: i32, sprite_height: i32) -> i32 {
        let board_height = sprite_sheet::board().height();
        let off_set_height = point_generator::center_side(board_height, self.screen_height);
        (row * (3 + sprite_height)) + 2 + off_set_height
    }

    fn board_square_y(&self, col: i32, sprite_width: i32) -> i32 {
        let board_width = sprite_sheet::board().width();
        let off_set_width = point_generator::center_side(board_width, self.screen_width);
        (col * (3 + sprite_width)) + 2 + off_set_width
    }

    fn token_spites(&self, board: Grid<BoardToken>) -> Vec<Sprite> {
        let mut tokens = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                let row_usize = row as usize;
                let col_uszie = col as usize;
                let sprite_data = board.clone().get(row_usize, col_uszie).to_sprite_data();
                let x = self.board_square_x(row, sprite_data.height());
                let y = self.board_square_y(col, sprite_data.width());
                let point = Point::new(x , y);
                tokens.push(Sprite::new(point, Color::WhiteOnBlue, sprite_data));
            }
        }
        tokens
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
