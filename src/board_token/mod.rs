use std::fmt::{Display, Formatter, Error};
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BoardToken {
    Blank,
    Player1,
    Player2,
}

impl Display for BoardToken {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        match *self {
            BoardToken::Player1 => write!(f, "X"),
            BoardToken::Player2 => write!(f, "O"),
            BoardToken::Blank => write!(f, " "),
        }
    }
}
