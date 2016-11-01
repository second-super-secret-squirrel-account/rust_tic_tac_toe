use game_board::board_token::BoardToken;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player {
    player_type: PlayerType,
    board_token: BoardToken,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PlayerType {
    Human,
    Computer,
}

impl Player {
    pub fn new(player_type: PlayerType, board_token: BoardToken) -> Player {
        Player {
            player_type: player_type,
            board_token: board_token,
        }
    }

    pub fn player_type(&self) -> &PlayerType {
        &self.player_type
    }

    pub fn board_token(&self) -> &BoardToken {
        &self.board_token
    }
}
