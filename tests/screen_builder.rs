extern crate tic_tac_toe;
use tic_tac_toe::game_board::grid::Grid;
use tic_tac_toe::game_board::board_token::BoardToken;
use tic_tac_toe::game_io::screen_builder::ScreenBuilder;


#[test]
fn screen_builder_returns_a_screen_with_a_background_sprite_size_50_100() {
    let board = Grid::new(3, 3, BoardToken::Blank);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board);

    let ref bg_sprite = screen.sprites()[0];
    let ref bg_sprite_data = bg_sprite.data();

    assert_eq!(50, bg_sprite_data.height());
    assert_eq!(100, bg_sprite_data.width());
}

#[test]
fn screen_builder_returns_a_screen_with_12_sprites_when_game_is_playing() {
    let board = Grid::new(3, 3, BoardToken::Blank);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board);

    assert_eq!(12, screen.sprites().len());
}


#[test]
fn screen_builder_returns_a_screen_with_13_sprites_when_game_is_over() {
    let mut board = Grid::new(3, 3, BoardToken::Blank);
    board.set(0, 0, BoardToken::PlayerX);
    board.set(0, 1, BoardToken::PlayerX);
    board.set(0, 2, BoardToken::PlayerX);
    let screen_builder = ScreenBuilder::new(50,100);
    let screen = screen_builder.playing_screen(board);

    assert_eq!(13, screen.sprites().len());
}
