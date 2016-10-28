extern crate tic_tac_toe;
use tic_tac_toe::game_io::grid::Grid;

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
use tic_tac_toe::game_io::grid::GridRow;

#[test]
fn test_that_grid_row_returns_size() {
    let grid_row = GridRow::new(3, ' ');
    assert_eq!(3, grid_row.len());
}

#[test]
fn test_that_grid_row_setter_getters_work() {
    let mut grid_row = GridRow::new(3, ' ');
    grid_row.set(1, 'a');
    assert_eq!('a', grid_row.get(1));
}
