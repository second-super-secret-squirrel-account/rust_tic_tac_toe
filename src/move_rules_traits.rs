use game_board_traits::GameBoard;

pub trait HasMoveRules<T> : GameBoard<T> {
    fn valid_move(&self, usize) -> bool;
    fn available_move_count(&self) -> usize;
    fn available_moves(&self) -> Vec<usize>;
}
