extern crate tic_tac_toe;

use tic_tac_toe::game_io::key_board_input::*;

// vim style movement keys
#[test]
fn is_up_returns_true_for_vim_binding() {
    assert!(is_up(KEY_K));
}

#[test]
fn is_down_returns_true_for_vim_binding() {
    assert!(is_down(KEY_J));
}

#[test]
fn is_left_returns_true_for_vim_binding() {
    assert!(is_left(KEY_H));
}

#[test]
fn is_right_returns_true_for_vim_binding() {
    assert!(is_right(KEY_L));
}

// WASD style movement keys
#[test]
fn is_up_returns_true_for_wasd_binding() {
    assert!(is_up(KEY_W));
}

#[test]
fn is_down_returns_true_for_wasd_binding() {
    assert!(is_down(KEY_S));
}

#[test]
fn is_left_returns_true_for_wasd_binding() {
    assert!(is_left(KEY_A));
}

#[test]
fn is_right_returns_true_for_wasd_binding() {
    assert!(is_right(KEY_D));
}

#[test]
fn is_select_returns_ture_for_space() {
    assert!(is_select(KEY_SPACEBAR));
}

#[test]
fn is_select_returns_false_for_non_spacebar_or_enter_key() {
    assert_eq!(false, is_select(KEY_Q));
}
