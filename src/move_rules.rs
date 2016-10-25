use game_board::Board;
use board_token::BoardToken;

pub fn valid_move(board: Board, space: usize) -> bool {
    *board.get_space(space) == BoardToken::Blank
}

pub fn moves_remaining(board: Board) -> usize {
    available_spaces(board).len()
}

pub fn available_spaces(board: Board) -> Vec<usize> {
    let mut open_spaces = Vec::new();
    for index in 1..(1 + board.size()) {
        if *board.get_space(index) == BoardToken::Blank {
            open_spaces.push(index);
        }
    }
        open_spaces
}
