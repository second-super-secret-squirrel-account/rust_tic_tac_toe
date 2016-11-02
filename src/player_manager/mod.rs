pub mod player;
pub mod mini_max;
use player_manager::player::Player;
use player_manager::player::PlayerType;
use game_board::board_token::BoardToken;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PlayerManager {
    player1: Player,
    player2: Player,
    current_player: Player,
}

impl PlayerManager {
    pub fn new(player1: Player, player2: Player) -> PlayerManager {
        PlayerManager {
            player1: player1.clone(),
            player2: player2,
            current_player: player1,
        }
    }

    pub fn reset(&mut self) {
        self.current_player = self.player1.clone();
    }

    pub fn current_player(&self) -> &Player {
        &self.current_player
    }

    pub fn non_current_player(&self) -> &Player {
        if self.current_player == self.player1 {
            return &self.player2;
        }
        &self.player1
    }

    pub fn change_turn(&mut self) {
        if self.current_player == self.player1 {
            self.current_player = self.player2.clone();
        } else {
            self.current_player = self.player1.clone();
        }
    }

    pub fn current_player_type(&self) -> &PlayerType {
        self.current_player.player_type()
    }

    pub fn current_player_token(&self) -> &BoardToken {
        self.current_player.board_token()
    }

    pub fn non_current_player_token(&self) -> &BoardToken {
        self.non_current_player().board_token()
    }
}
