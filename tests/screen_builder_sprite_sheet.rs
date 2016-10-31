extern crate tic_tac_toe;
use tic_tac_toe::game_io::screen_builder::sprite_sheet::background;

#[test]
fn background_sprite_is_the_size_of_screen() {
    let data = background(50, 100);

    assert_eq!(50, data.height());
    assert_eq!(100, data.width());

}
