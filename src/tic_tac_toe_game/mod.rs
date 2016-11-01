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
use mini_max::MiniMax;
use player_manager::PlayerManager;
use player_manager::player::PlayerType;

pub struct TicTacToeGame {
    player_manager: PlayerManager,
    board: Grid<BoardToken>,
    active: bool,
}

impl TicTacToeGame {

    pub fn new(player_manager: PlayerManager) -> TicTacToeGame {
        let board = Grid::new(3, 3, BoardToken::Blank);
        TicTacToeGame {
            player_manager: player_manager,
            board: board,
            active: true,
        }
    }

    pub fn computer_turn(&mut self) {
        let cpu_move = MiniMax::get_move(self.board.clone(),
                                         self.player_manager.current_player_token().clone(),
                                         self.player_manager.non_current_player_token().clone());
        let token = self.player_manager.current_player().board_token().clone();
        self.board.set_space(cpu_move, token);
        self.player_manager.change_turn();
    }

    pub fn human_turn(&mut self) {
        let key = get_key() - 48;
        let key = key as usize;
        if key < 9 {
            if self.board.valid_move(key) {
                let token = self.player_manager.current_player_token().clone();
                self.board.set_space(key, token);
                self.player_manager.change_turn();
            }
        }
    }

    pub fn draw_screen(&mut self) {
        let screen_builder = ScreenBuilder::new(screen_height(), screen_width());
        let screen = screen_builder.playing_screen(self.board.clone());
        screen.draw_screen();
    }

    pub fn game_over(&mut self) {
        let key = get_key();
        if key == KEY_R {
            self.board = Grid::new(3, 3, BoardToken::Blank);
        } else if key == KEY_Q {
            self.active = false;
        }
    }

    pub fn play(&mut self) {
        while self.active {
            while self.board.game_status() == GameState::Playing {
                self.draw_screen();
                if *self.player_manager.current_player_type() == PlayerType::Human {
                    self.human_turn();
                } else {
                    self.computer_turn();
                }
            }
            self.draw_screen();
            self.game_over();
        }
    }
}
