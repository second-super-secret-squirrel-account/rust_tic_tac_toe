extern crate std;
use game_board::game_board::GameBoard;
use game_board::basic_board_traits::BasicBoard;
use game_rules::game_state::GameState;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::move_rules_traits::HasMoveRules;
use game_board::properties_traits::HasProperties;

impl <T: Clone + PartialEq> HasGameStatus<T> for GameBoard<T> {
    fn game_status(&self) -> GameState {
        if self.is_there_a_winner() {
            return GameState::Winner;
        }

        if self.available_moves_count() == 0 {
            return GameState::Tied;
        }
        GameState::Playing
    }
}

impl <T: Clone + PartialEq> GameBoard<T> {
    fn is_there_a_winner(&self) -> bool {
        let combinations = self.generate_winning_combinations();
        for combination in combinations {
            if self.get_space(combination[0]) != self.blank_value() && self.check_if_spaces_are_equal(combination) {
                return true;
            }
        }
        false
    }

    fn generate_winning_combinations(&self) -> Vec<Vec<usize>> {
        let mut combinations = Vec::new();
        if self.size() == 9 {
            combinations.push(vec!(1, 2, 3));
            combinations.push(vec!(4, 5, 6));
            combinations.push(vec!(7, 8, 9));
            combinations.push(vec!(1, 4, 7));
            combinations.push(vec!(2, 5, 8));
            combinations.push(vec!(3, 6, 9));
            combinations.push(vec!(1, 5, 9));
            combinations.push(vec!(3, 5, 7));
        } else if self.size() == 16 {
            combinations.push(vec!(1, 2, 3, 4));
            combinations.push(vec!(5, 6, 7, 8));
            combinations.push(vec!(9, 10, 11, 12));
            combinations.push(vec!(13, 14, 15, 16));
            combinations.push(vec!(1, 5, 9, 13));
            combinations.push(vec!(2, 6, 10, 14));
            combinations.push(vec!(3, 7, 11, 15));
            combinations.push(vec!(4, 8, 12, 16));
            combinations.push(vec!(1, 6, 11, 16));
            combinations.push(vec!(4, 7, 10, 13));
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
