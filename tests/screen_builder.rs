extern crate tic_tac_toe;
use tic_tac_toe::game_board::grid::Grid;
use tic_tac_toe::game_board::board_token::BoardToken;
use tic_tac_toe::game_io::screen_builder::ScreenBuilder;
use tic_tac_toe::tic_tac_toe_game::cursor::Cursor;
use tic_tac_toe::game_io::game_screen::sprite::color::Color;


#[test]
fn playing_screen_returns_a_screen_with_a_background_sprite_size_50_100() {
    let board = Grid::new(3, 3, BoardToken::Blank);
    let cursor = Cursor::new();
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board, cursor);

    let ref bg_sprite = screen.sprites()[0];
    let ref bg_sprite_data = bg_sprite.data();

    assert_eq!(50, bg_sprite_data.height());
    assert_eq!(100, bg_sprite_data.width());
}

#[test]
fn playing_screen_returns_a_screen_with_12_sprites_when_game_is_playing() {
    let board = Grid::new(3, 3, BoardToken::Blank);
    let cursor = Cursor::new();
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board, cursor);

    assert_eq!(13, screen.sprites().len());
}


#[test]
fn playing_screen_returns_a_screen_with_15_sprites_when_game_is_over() {
    let mut board = Grid::new(3, 3, BoardToken::Blank);
    let cursor = Cursor::new();
    board.set(0, 0, BoardToken::PlayerX);
    board.set(0, 1, BoardToken::PlayerX);
    board.set(0, 2, BoardToken::PlayerX);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board, cursor);

    assert_eq!(15, screen.sprites().len());
}

#[test]
fn setup_screen_returns_a_screen_with_7_sprites() {
    let cursor = Cursor::new();
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.setup_screen(cursor);
    assert_eq!(7, screen.sprites().len());
}

#[test]
fn setup_screen_highlights_menu_item_1_only_when_cursor_row_1() {
    let mut cursor = Cursor::new();
    cursor.set_space(0);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.setup_screen(cursor);
    assert_eq!(Color::BlueOnYellow, * screen.sprites()[2].color());
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[3].color());
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[4].color());
}

#[test]
fn setup_screen_highlights_menu_item_1_only_when_cursor_row_2() {
    let mut cursor = Cursor::new();
    cursor.set_space(3);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.setup_screen(cursor);
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[2].color());
    assert_eq!(Color::BlueOnYellow, * screen.sprites()[3].color());
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[4].color());
}

#[test]
fn setup_screen_highlights_menu_item_1_only_when_cursor_row_3() {
    let mut cursor = Cursor::new();
    cursor.set_space(6);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.setup_screen(cursor);
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[2].color());
    assert_eq!(Color::YellowOnBlue, * screen.sprites()[3].color());
    assert_eq!(Color::BlueOnYellow, * screen.sprites()[4].color());
}
