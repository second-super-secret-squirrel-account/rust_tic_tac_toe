use game_board_traits::GameBoard;

pub trait HasProperties<T> : GameBoard<T> {
    fn size(&self) -> usize;
//    fn available_move_count(&self) -> usize;
    fn row_size(&self) -> usize;
    fn corners(&self) -> [usize; 4];
}
