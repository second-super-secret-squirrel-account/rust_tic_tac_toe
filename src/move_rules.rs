use game_board::Board;
use board_token::BoardToken;

pub fn valid_move(board: Board, space: usize) -> bool {
    *board.get_space(space) == BoardToken::BLANK
}

pub fn moves_remaining(board: Board) -> usize {
    let mut blank_spaces = 0;
    for space in board.spaces() {
        if *space == BoardToken::BLANK {
            blank_spaces += 1;
        }
    }
    blank_spaces
}

pub fn available_spaces(board: Board) -> Vec<usize> {
    let mut open_spaces = Vec::new();
    for index in 1..(1 + board.size()) {
        if *board.get_space(index) == BoardToken::BLANK {
            open_spaces.push(index);
        }
    }
        open_spaces
}
