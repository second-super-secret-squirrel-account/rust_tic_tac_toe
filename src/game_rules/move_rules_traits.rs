use game_board::basic_board_traits::BasicBoard;

pub trait HasMoveRules<T> : BasicBoard<T> {
    fn valid_move(&self, usize) -> bool;
    fn available_moves_count(&self) -> usize;
    fn available_moves(&self) -> Vec<usize>;
}
