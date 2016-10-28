extern crate tic_tac_toe;
use tic_tac_toe::grid::Grid;

#[test]
fn test_grid_setter_and_getter() {
    let mut test_grid = Grid::new(3, 3, ' ');

    test_grid.set(1, 1, 'a');

    assert_eq!('a', test_grid.get(1, 1));
}

#[test]
fn test_rows_returns_2_on_a_2x4_grid() {
    let mut test_grid = Grid::new(2, 4, ' ');

    assert_eq!(2, test_grid.rows());
}

#[test]
fn test_cols_returns_4_on_a_2x4_grid() {
    let mut test_grid = Grid::new(2, 4, ' ');

    assert_eq!(4, test_grid.cols());
}

#[test]
fn fill_value_getter_returns_value_passed_to_constructor() {
    let test_grid = Grid::new(3, 3, ' ');

    assert_eq!(' ', test_grid.fill_value());
}
