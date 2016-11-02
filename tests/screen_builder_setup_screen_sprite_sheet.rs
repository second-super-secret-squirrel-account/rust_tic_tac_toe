extern crate tic_tac_toe;

use tic_tac_toe::game_io::screen_builder::setup_screen::sprite_sheet::menu_item;

#[test]
fn menu_item_0_is_human_vs_human() {
    let test_menu_item = menu_item(0);
    assert_eq!("Human      vs     Human   ", test_menu_item.line(0));
}

#[test]
fn menu_item_1_is_human_vs_human() {
    let test_menu_item = menu_item(1);
    assert_eq!("Human      vs     Computer", test_menu_item.line(0));
}

#[test]
fn menu_item_2_is_human_vs_human() {
    let test_menu_item = menu_item(2);
    assert_eq!("Computer   vs     Human   ", test_menu_item.line(0));
}
