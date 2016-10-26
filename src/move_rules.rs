use game_board::Board;
use game_board_traits::GameBoard;
use move_rules_traits::HasMoveRules;

impl <T: Clone + PartialEq> HasMoveRules<T> for Board<T> {
    fn available_move_count(&self) -> usize {
        self.available_moves().len()
    }


    fn valid_move(&self, space: usize) -> bool {
        self.get_space(space) == self.blank_value()
    }

    fn available_moves(&self) -> Vec<usize> {
        let mut open_spaces = Vec::new();
        for index in 1..(1 + &self.spaces().len()) {
            if *self.get_space(index) == *self.blank_value() {
                open_spaces.push(index);
            }
        }
        open_spaces
    }
}
