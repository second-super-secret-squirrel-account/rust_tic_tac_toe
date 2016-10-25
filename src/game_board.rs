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

    pub fn spaces(&self) -> &Vec<BoardToken> {
        &self.spaces
    }

    pub fn size(&self) -> usize {
        self.spaces.len()
    }

    pub fn corners(&self) -> [usize; 4] {
        let row = self.row_size();
        let mut corners = [1; 4];
        corners[1] = row;
        corners[2] = row * (row - 1);
        corners[3] = row * row;
        corners
    }

    pub fn row_size(&self) -> usize {
        let size = self.size() as f32;
        size.sqrt() as usize
    }

    pub fn set_space(&mut self, space: usize, value: BoardToken) {
        if self.valid_board_index(space) {
            self.spaces[space - 1] = value;
        }
    }

    pub fn get_space(&self, space: usize) -> &BoardToken {
            return &self.spaces[space - 1];
    }

    fn valid_board_index(&self, index: usize) -> bool {
        index > 0 && index <= self.spaces.len()
    }
}

