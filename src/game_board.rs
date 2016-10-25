use board_token::BoardToken;

pub struct Board {
    spaces: Vec<BoardToken>
}

impl Board
{
    pub fn new(size: usize) -> Board {
        Board {
            spaces: vec![BoardToken::BLANK; size],
        }
    }

    pub fn set_space(&mut self, space: usize, value: BoardToken) {
        let size = self.spaces.len();
        if space > 0 && space <= size {
            self.spaces[space - 1] = value;
        }
    }

    pub fn get_space(&self, space: usize) -> &BoardToken {
        &self.spaces[space - 1]
    }
}

