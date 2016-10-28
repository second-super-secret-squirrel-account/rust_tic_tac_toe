use grid::Grid;
use game_board::game_board_traits::GameBoard;
use game_rules::move_rules_traits::HasMoveRules;

impl <T: Clone + PartialEq> HasMoveRules<T> for Grid<T> {
    fn available_moves_count(&self) -> usize {
        self.available_moves().len()
    }

    fn valid_move(&self, space: usize) -> bool {
        self.get_space(space) == self.fill_value()
    }

    fn available_moves(&self) -> Vec<usize> {
        let mut open_spaces = Vec::new();
        for index in 0..(self.size()) {
            if self.get_space(index) == self.fill_value() {
                open_spaces.push(index);
            }
        }
        open_spaces
    }
}
