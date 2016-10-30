mod sprite_sheet;


use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;
use game_io::screen_width;
use game_io::screen_height;

pub fn playing_screen() -> GameScreen {
    let mut screen = GameScreen::new();


    let screen_width = screen_width();
    let screen_height = screen_height();

    let board_width = sprite_sheet::board().width();
    let board_height = sprite_sheet::board().height();

    let board_x = (screen_height / 2) - (board_height / 2);
    let board_y = (screen_width / 2) - (board_width / 2);

    let point = Point::new(board_x, board_y);

    let board_sprite = Sprite::new(point, Color::WhiteOnBlue, sprite_sheet::board());

    screen.add_sprite(board_sprite);

    screen

}
