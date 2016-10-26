use game_board::Board;
use game_board_traits::GameBoard;
use board_properties_traits::HasProperties;

impl <T: Clone + PartialEq > HasProperties<T> for Board<T> {
    fn size(&self) -> usize {
        self.spaces().len()
    }

    fn row_size(&self) -> usize {
        let size = self.size() as f32;
        size.sqrt() as usize
    }

    fn corners(&self) -> [usize; 4] {
        let row = self.row_size();
        let mut corners = [1; 4];
        corners[1] = row;
        corners[2] = row * (row - 1);
        corners[3] = row * row;
        corners
    }
}
