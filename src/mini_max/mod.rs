use std::fmt;
use grid::Grid;
use game_rules::game_state::GameState;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::move_rules_traits::HasMoveRules;
use game_board::game_board_traits::GameBoard;

static STARTING_DEPTH: i32 = 0;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MiniMax<T> {
    current_player: T,
    opponent: T,

}

impl <T: Clone + PartialEq> MiniMax<T> where T: fmt::Display {
    pub fn get_move(board: Grid<T>, current_player_token: T, opponent_token: T) -> usize {
        let mini_max = MiniMax {
            current_player: current_player_token.clone(),
            opponent: opponent_token.clone(),
        };
        let moves = board.available_moves();
        let mut scores = Vec::new();
        for space in moves.clone() {
            let mut temp_board = board.clone();
            temp_board.set_space(space, current_player_token.clone());
            scores.push(mini_max.mini_max(temp_board,
                                              STARTING_DEPTH,
                                              opponent_token.clone(),
                                              current_player_token.clone()));
        }
        let best_score_index = get_index_of_highest_score(scores);
        moves[best_score_index]
    }



    fn mini_max(&self, board: Grid<T>, depth: i32, current_player: T, non_current_player: T) -> i32 {
        if board.game_status() != GameState::Playing {
            return self.score(board, depth);
        }
        let moves = board.available_moves();
        let mut scores = Vec::new();
        for space in moves.clone() {
            let mut temp_board = board.clone();
            temp_board.set_space(space, current_player.clone());
            scores.push(self.mini_max(temp_board,
                                     depth + 1,
                                     non_current_player.clone(),
                                     current_player.clone()));
        }
        self.get_best_score(scores, current_player)
    }

    fn get_best_score(&self, scores: Vec<i32>, player: T) -> i32 {
        if player == self.current_player {
            let index = get_index_of_highest_score(scores.clone());
            return scores[index];
        } else {
            let index = get_index_of_lowest_score(scores.clone());
            return scores[index];
        }
    }

    fn score(&self, board: Grid<T>, depth: i32) -> i32 {
        if board.game_winner() == self.current_player {
            return 100 - depth;
        } else if board.game_winner() == self.opponent {
            return depth - 100;
        } else {
            return 0 - depth;
        }
    }
}

fn get_index_of_highest_score(scores: Vec<i32>) -> usize {
    if scores.len() == 0 {
        return 0;
    }
    let mut return_index = 0;
    let mut max_value = scores[0];
    for index in 1..scores.len() {
        if scores[index] > max_value {
            max_value = scores[index];
            return_index = index;
        }
    }
    return_index
}

fn get_index_of_lowest_score(scores: Vec<i32>) -> usize {
    if scores.len() == 0 {
        return 0;
    }
    let mut return_index = 0;
    let mut min_value = scores[0];
    for index in 1..scores.len() {
        if scores[index] < min_value {
            min_value = scores[index];
            return_index = index;
        }
    }
    return_index
}
