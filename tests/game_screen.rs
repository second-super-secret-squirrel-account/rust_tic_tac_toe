extern crate tic_tac_toe;
mod test_helper;
use test_helper::*;
use tic_tac_toe::game_io::game_screen::GameScreen;


#[test]
fn test_add_sprite_increases_the_sprite_count() {
    let mut game_screen = GameScreen::new();

    let sprite = test_sprite();
    game_screen.add_sprite(sprite);

    assert_eq!(1, game_screen.sprites().len());

}
