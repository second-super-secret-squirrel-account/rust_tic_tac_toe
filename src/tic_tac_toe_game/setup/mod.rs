use game_io::key_board_input::*;
use game_io::screen_width;
use game_io::screen_height;
use tic_tac_toe_game::cursor::Cursor;
use game_io::screen_builder::ScreenBuilder;
use tic_tac_toe_game::TicTacToeGame;
use player_manager::player::PlayerType;
use player_manager::player::Player;
use player_manager::PlayerManager;
use game_board::board_token::BoardToken;

pub struct TicTacToeGameSetup {
    cursor: Cursor,
    active: bool,
}

impl TicTacToeGameSetup {
    pub fn new() -> TicTacToeGameSetup {
        let cursor = Cursor::new();
        TicTacToeGameSetup {
            cursor: cursor,
            active: true,
        }
    }

    pub fn run(&mut self) {
        self.cursor.set_space(0);
        while self.active {
            self.draw_screen();
            let key = get_key();
            if is_select(key) {
                let mut game = self.create_game();
                game.play();
            } else {
                self.process_key(key);
            }
        }
    }

    fn draw_screen(&mut self) {
        let screen_builder = ScreenBuilder::new(screen_height(), screen_width());
        let screen = screen_builder.setup_screen(self.cursor.clone());
        screen.draw_screen();
    }

    fn create_game(&self) -> TicTacToeGame {
        let player_manager = self.create_player_manager();
        TicTacToeGame::new(player_manager)
    }

    fn create_player_manager(&self) -> PlayerManager {
        let mut player_type_1 = PlayerType::Human;
        let mut player_type_2 = PlayerType::Human;

        if self.cursor.row() == 1 {
            player_type_2 = PlayerType::Computer;
        } else if self.cursor.row() == 2 {
            player_type_1 = PlayerType::Computer;
        }

        let player_1 = Player::new(player_type_1, BoardToken::PlayerX);
        let player_2 = Player::new(player_type_2, BoardToken::PlayerO);
        PlayerManager::new(player_1, player_2)
    }

    fn process_key(&mut self, key: i32) {
        if is_up(key) {
            self.cursor.move_up();
        } else if is_down(key) {
            self.cursor.move_down();
        } else if key == KEY_Q {
            self.active = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use player_manager::player::PlayerType;
    use game_io::key_board_input::*;
    use super::*;

    #[test]
    fn process_key_sets_active_to_false_on_key_q() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.process_key(KEY_Q);

        assert_eq!(false, test_setup.active);
    }

    #[test]
    fn process_key_moves_cursor_down_on_key_j() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.process_key(KEY_J);

        assert_eq!(2, test_setup.cursor.row());
    }

    #[test]
    fn process_key_moves_cursor_up_on_key_k() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.process_key(KEY_K);

        assert_eq!(0, test_setup.cursor.row());
    }

    #[test]
    fn create_player_manager_returns_human_vs_human_when_cursor_is_zero() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.cursor.set_space(0);
        let mut player_manager = test_setup.create_player_manager();

        assert_eq!(PlayerType::Human, *player_manager.current_player_type());
        player_manager.change_turn();
        assert_eq!(PlayerType::Human, *player_manager.current_player_type());
    }

    #[test]
    fn create_player_manager_returns_human_vs_computer_when_cursor_is_one() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.cursor.set_space(3);
        let mut player_manager = test_setup.create_player_manager();

        assert_eq!(PlayerType::Human, *player_manager.current_player_type());
        player_manager.change_turn();
        assert_eq!(PlayerType::Computer, *player_manager.current_player_type());
    }

    #[test]
    fn create_player_manager_returns_computer_vs_human_when_cursor_is_two() {
        let mut test_setup = TicTacToeGameSetup::new();

        test_setup.cursor.set_space(6);
        let mut player_manager = test_setup.create_player_manager();

        assert_eq!(PlayerType::Computer, *player_manager.current_player_type());
        player_manager.change_turn();
        assert_eq!(PlayerType::Human, *player_manager.current_player_type());
    }
}
