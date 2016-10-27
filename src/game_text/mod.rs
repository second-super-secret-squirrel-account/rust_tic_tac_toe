use board_token::BoardToken;

pub struct GameText {
    title: String,
    winner_is: String,
    tied: String,
    mark1: char,
    mark2: char,
    }


impl GameText {

    pub fn new(mark1: char, mark2: char) -> GameText {
        GameText {
            mark1: mark1,
            mark2: mark2,
            title: "TIC TAC TOE".to_string(),
            winner_is: "The winner is player".to_string(),
            tied: "Game ended in a Tie!".to_string(),

        }
    }

    pub fn title(&self) -> String {
        &self.title;
    }

    pub fn tied(&self) -> String {
        &self.tied;
    }

    pub fn winner_string(&self, winner: BoardToken) -> char {
        let mark = match winner {
            BoardToken::Player1 => self.mark1,
            BoardToken::Player2 => self.mark2,
            _ => ' ',
        };
        println!("{} {}", self.winner_is, mark);
        mark
    }
}
