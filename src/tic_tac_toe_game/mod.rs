pub mod cursor;
pub mod setup;

use game_io::screen_builder::ScreenBuilder;
use game_io::key_board_input::*;
use game_io::screen_width;
use game_io::screen_height;
use game_board::grid::Grid;
use game_board::board_token::BoardToken;
use game_rules::move_rules_traits::HasMoveRules;
use game_board::game_board_traits::GameBoard;
use game_rules::game_status_traits::HasGameStatus;
use game_rules::game_state::GameState;
use player_manager::mini_max::MiniMax;
use player_manager::PlayerManager;
use player_manager::player::PlayerType;
use tic_tac_toe_game::cursor::Cursor;



pub struct TicTacToeGame {
    player_manager: PlayerManager,
    board: Grid<BoardToken>,
    cursor: Cursor,
    active: bool,
}

impl TicTacToeGame {

    pub fn new(player_manager: PlayerManager) -> TicTacToeGame {
        let board = Grid::new(3, 3, BoardToken::Blank);
        let cursor = Cursor::new();
        TicTacToeGame {
            player_manager: player_manager,
            board: board,
            cursor: cursor,
            active: true,
        }
    }

    fn computer_turn(&mut self) {
        let cpu_move = MiniMax::get_move(self.board.clone(),
                                         self.player_manager.current_player_token().clone(),
                                         self.player_manager.non_current_player_token().clone());
        let token = self.player_manager.current_player().board_token().clone();
        self.board.set_space(cpu_move, token);
        self.player_manager.change_turn();
        self.cursor.set_space(cpu_move);
    }

    fn update_cursor_position(&mut self, key: i32) {
        if is_up(key) {
            self.cursor.move_up();
        } else if is_down(key) {
            self.cursor.move_down();
        } else if is_left(key) {
            self.cursor.move_left();
        } else if is_right(key) {
            self.cursor.move_right();
        }
    }

    fn play_human_move(&mut self) {
        let token = self.player_manager.current_player_token().clone();
        let space = self.cursor.index();
        if self.board.valid_move(space) {
            self.board.set_space(space, token);
            self.player_manager.change_turn();
        }
    }

    fn human_turn(&mut self, key: i32) {
        self.update_cursor_position(key);
        if is_select(key) {
            self.play_human_move();
        }
    }

    fn play_again(&mut self, key: i32) {
        if key == KEY_R {
            self.board = Grid::new(3, 3, BoardToken::Blank);
            self.player_manager.reset();
            self.cursor.set_space(4);
        } else if key == KEY_Q {
            self.active = false;
        }
    }

    fn draw_screen(&mut self) {
        let screen_builder = ScreenBuilder::new(screen_height(), screen_width());
        let screen = screen_builder.playing_screen(self.board.clone(), self.cursor.clone());
        screen.draw_screen();
    }

    pub fn play(&mut self) {
        while self.active {
            while self.board.game_status() == GameState::Playing {
                self.draw_screen();
                if *self.player_manager.current_player_type() == PlayerType::Human {
                    self.human_turn(get_key());
                } else {
                    self.computer_turn();
                }
            }

            self.draw_screen();
            self.play_again(get_key());
        }
    }
}

#[cfg(test)]
mod tests {
    use player_manager::player::PlayerType;
    use player_manager::player::Player;
    use player_manager::PlayerManager;
    use game_board::board_token::BoardToken;
    use game_io::key_board_input::*;
    use super::*;

    #[test]
    fn human_turn_plays_move_center_when_key_is_enter_and_cursor_is_center_space() {
        let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
        let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
        let player_manager = PlayerManager::new(player1, player2);
        let mut test_game = TicTacToeGame::new(player_manager);

        test_game.human_turn(KEY_SPACEBAR);

        assert_eq!(BoardToken::PlayerX, test_game.board.get(1, 1));
    }

    #[test]
    fn human_turn_moves_cursor_up_when_k_is_pressed() {
        let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
        let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
        let player_manager = PlayerManager::new(player1, player2);
        let mut test_game = TicTacToeGame::new(player_manager);

        test_game.human_turn(KEY_K);

        assert_eq!(1, test_game.cursor.index());
    }

    #[test]
    fn human_turn_moves_cursor_down_when_j_is_pressed() {
        let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
        let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
        let player_manager = PlayerManager::new(player1, player2);
        let mut test_game = TicTacToeGame::new(player_manager);

        test_game.human_turn(KEY_J);

        assert_eq!(7, test_game.cursor.index());
    }

    #[test]
    fn play_again_sets_active_to_false_when_q_is_pressed() {
        let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
        let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
        let player_manager = PlayerManager::new(player1, player2);
        let mut test_game = TicTacToeGame::new(player_manager);

        test_game.play_again(KEY_Q);

        assert_eq!(false, test_game.active);
    }

    #[test]
    fn play_again_sets_start_new_game_when_r_is_pressed() {
        let player1 = Player::new(PlayerType::Human, BoardToken::PlayerX);
        let player2 = Player::new(PlayerType::Computer, BoardToken::PlayerO);
        let player_manager = PlayerManager::new(player1, player2);
        let mut test_game = TicTacToeGame::new(player_manager);

        test_game.human_turn(KEY_ENTER);
        assert_eq!(BoardToken::PlayerX, test_game.board.get(1, 1));
        test_game.play_again(KEY_R);
        assert_eq!(BoardToken::Blank, test_game.board.get(1, 1));

    }
}
