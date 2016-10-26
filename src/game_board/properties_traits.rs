use game_board::basic_board_traits::BasicBoard;

pub trait HasProperties<T> : BasicBoard<T> {
    fn size(&self) -> usize;
    fn row_size(&self) -> usize;
    fn corners(&self) -> [usize; 4];
}
