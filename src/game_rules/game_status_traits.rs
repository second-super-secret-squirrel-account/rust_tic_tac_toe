use game_rules::game_state::GameState;
use game_rules::move_rules_traits::HasMoveRules;

pub trait HasGameStatus<T> : HasMoveRules<T> {
    fn game_status(&self) -> GameState;
    fn game_winner(&self) -> T;
}
