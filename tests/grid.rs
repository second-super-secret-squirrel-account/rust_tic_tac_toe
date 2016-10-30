extern crate tic_tac_toe;
use tic_tac_toe::game_board::grid::Grid;

#[test]
fn test_grid_set_and_get() {
    let mut test_grid = Grid::new(3, 3, ' ');

    test_grid.set(1, 1, 'a');

    assert_eq!('a', test_grid.get(1, 1));
}

#[test]
fn test_rows_returns_2_on_a_2x4_grid() {
    let test_grid = Grid::new(2, 4, ' ');

    assert_eq!(2, test_grid.rows());
}

#[test]
fn test_cols_returns_4_on_a_2x4_grid() {
    let test_grid = Grid::new(2, 4, ' ');

    assert_eq!(4, test_grid.cols());
}

#[test]
fn test_len_returns_9_on_a_3x3_grid() {
    let test_grid = Grid::new(3, 3, ' ');

    assert_eq!(9, test_grid.size());
}

#[test]
fn fill_value_getter_returns_value_passed_to_constructor() {
    let test_grid = Grid::new(3, 3, ' ');

    assert_eq!(' ', test_grid.fill_value());
}
