extern crate tic_tac_toe;

use tic_tac_toe::grid::Grid;
use tic_tac_toe::sprite::sprite_data::SpriteData;
use tic_tac_toe::sprite::sprite_data_traits::*;
use tic_tac_toe::board_token::BoardToken;

#[test]
fn returns_an_empty_board() {
    let test_grid = Grid::new(3, 3, BoardToken::Blank);
    let sprite_data = test_grid.to_sprite_data();
    let mut expected = SpriteData::new();
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    assert_eq!(expected, sprite_data);
}

#[test]
fn returns_a_filled_out_board() {
    let mut test_grid = Grid::new(3, 3, ' ');
    for i in 0..3 {
        for j in 0..3 {
            test_grid.set(i, j, 'X');
        }
    }
    let sprite_data = test_grid.to_sprite_data();

    let mut expected = SpriteData::new();
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  X  |  X  |  X  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  X  |  X  |  X  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  X  |  X  |  X  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    assert_eq!(expected, sprite_data);
}

#[test]
fn returns_a_filled_out_board_of_ints() {
    let mut test_grid = Grid::new(3, 3, 0);
    for i in 0..3 {
        for j in 0..3 {
            let value = ((i * 3) + j);
            test_grid.set(i, j, value);
        }
    }
    let sprite_data = test_grid.to_sprite_data();

    let mut expected = SpriteData::new();
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  0  |  1  |  2  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  3  |  4  |  5  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line("|  6  |  7  |  8  |".to_string());
    expected.add_line("|     |     |     |".to_string());
    expected.add_line(" ----- ----- ----- ".to_string());
    assert_eq!(expected, sprite_data);
}
