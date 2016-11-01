use std::fmt::{Display, Formatter, Error};
mod drawable;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BoardToken {
    Blank,
    PlayerX,
    PlayerO,
}

impl Display for BoardToken {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        match *self {
            BoardToken::PlayerX => write!(f, "X"),
            BoardToken::PlayerO => write!(f, "O"),
            BoardToken::Blank => write!(f, " "),
        }
    }
}
