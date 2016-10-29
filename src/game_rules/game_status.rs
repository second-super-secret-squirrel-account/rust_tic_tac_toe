extern crate std;
use grid::Grid;
use game_board::game_board_traits::GameBoard;
use game_rules::game_state::GameState;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::move_rules_traits::HasMoveRules;

impl <T: Clone + PartialEq> HasGameStatus<T> for Grid<T> {
    fn game_status(&self) -> GameState {
        if self.game_winner() != self.fill_value() {
            return GameState::Winner;
        }

        if self.available_moves_count() == 0 {
            return GameState::Tied;
        }
        GameState::Playing
    }

    fn game_winner(&self) -> T {
        self.is_there_a_winner()
    }
}

impl <T: Clone + PartialEq> Grid<T> {
    fn is_there_a_winner(&self) -> T {
        let combinations = self.generate_winning_combinations();
        for combination in combinations {
            let mut current_mark = self.get_space(combination.clone()[0]);
            //current_mark = self.get_space(combination[0]);
            if self.get_space(combination[0]) != self.fill_value() && self.check_if_spaces_are_equal(combination) {
                return current_mark;
            }
        }
        return self.fill_value();
    }

    fn generate_winning_combinations(&self) -> Vec<Vec<usize>> {
        let mut combinations = Vec::new();
        if self.size() == 9 {
            combinations.push(vec!(0, 1, 2));
            combinations.push(vec!(3, 4, 5));
            combinations.push(vec!(6, 7, 8));
            combinations.push(vec!(0, 3, 6));
            combinations.push(vec!(1, 4, 7));
            combinations.push(vec!(2, 5, 8));
            combinations.push(vec!(0, 4, 8));
            combinations.push(vec!(2, 4, 6));
        } else if self.size() == 16 {
            combinations.push(vec!(0, 1, 2, 3));
            combinations.push(vec!(4, 5, 6, 7));
            combinations.push(vec!(8, 9, 10, 11));
            combinations.push(vec!(12, 13, 14, 15));
            combinations.push(vec!(0, 4, 8, 12));
            combinations.push(vec!(1, 5, 9, 13));
            combinations.push(vec!(2, 6, 10, 14));
            combinations.push(vec!(3, 7, 11, 15));
            combinations.push(vec!(2, 5, 10, 15));
            combinations.push(vec!(3, 6, 9, 12));
        }
        combinations
    }

    fn check_if_spaces_are_equal(&self, spaces: Vec<usize>) -> bool {
        let compare = spaces.clone()[0];
        let compare = self.get_space(compare);
        for space in spaces {
           if self.get_space(space) != compare {
                return false;
            }
        }
        true
    }
}
