extern crate tic_tac_toe;

use tic_tac_toe::tic_tac_toe_game::cursor::Cursor;

#[test]
fn constructor_sets_cursor_row_col_to_1_1_on_3x3_cursor() {
    let cursor = Cursor::new();

    assert_eq!(1, cursor.row());
    assert_eq!(1, cursor.col());
}

#[test]
fn move_left_decreases_col_index() {
    let mut cursor = Cursor::new();

    cursor.move_left();

    assert_eq!(0, cursor.col());
}

#[test]
fn col_can_not_go_below_zero() {
    let mut cursor = Cursor::new();

    cursor.move_left();
    cursor.move_left();

    assert_eq!(0, cursor.col());
}

#[test]
fn move_right_increases_col_index() {
    let mut cursor = Cursor::new();

    cursor.move_right();

    assert_eq!(2, cursor.col());
}

#[test]
fn col_can_not_go_above_2() {
    let mut cursor = Cursor::new();

    cursor.move_right();
    cursor.move_right();

    assert_eq!(2, cursor.col());
}


#[test]
fn move_up_decreases_row_index() {
    let mut cursor = Cursor::new();

    cursor.move_up();

    assert_eq!(0, cursor.row());
}

#[test]
fn row_can_not_go_below_zero() {
    let mut cursor = Cursor::new();

    cursor.move_up();
    cursor.move_up();

    assert_eq!(0, cursor.row());
}

#[test]
fn move_down_increases_row_index() {
    let mut cursor = Cursor::new();

    cursor.move_down();

    assert_eq!(2, cursor.row());
}

#[test]
fn row_can_not_go_above_2() {
    let mut cursor = Cursor::new();

    cursor.move_down();
    cursor.move_down();

    assert_eq!(2, cursor.row());
}

#[test]
fn get_index_returns_4_when_row_1_col_1() {
    let cursor = Cursor::new();

    assert_eq!(4, cursor.index());
}

#[test]
fn get_index_returns_7_when_row_2_col_1() {
    let mut cursor = Cursor::new();

    cursor.move_down();

    assert_eq!(7, cursor.index());
}
