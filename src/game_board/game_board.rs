use grid::Grid;
use game_board::game_board_traits::GameBoard;

impl <T: Clone + PartialEq> GameBoard<T> for Grid<T> {
    fn corners(&self) -> [usize; 4] {
        let row = self.rows();
        let mut corners = [1; 4];
        corners[1] = row;
        corners[2] = row * (row - 1);
        corners[3] = row * row;
        corners
    }

    fn set_space(&mut self, space: usize, value: T) {
        let row = self.space_to_row(space);
        let col = self.space_to_col(space);
        self.set(row, col, value);
    }

    fn get_space(&self, space: usize) -> T {
        let row = self.space_to_row(space);
        let col = self.space_to_col(space);
        self.get(row, col)
    }
}

impl <T: Clone + PartialEq> Grid<T> {
    fn space_to_row(&self, space: usize) -> usize {
        space / self.rows()
    }

    fn space_to_col(&self, space: usize) -> usize {
        space % self.cols()
    }
}
