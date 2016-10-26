use game_board::basic_board_traits::BasicBoard;

pub struct GameBoard<T> {
    blank_value: T,
    spaces: Vec<T>,
}

impl<T: Clone + PartialEq> GameBoard<T> {
    pub fn new(size: usize, fill: T) -> GameBoard<T> {
        let blank = fill.clone();
        let size = size * size;
        GameBoard {
            blank_value: fill,
            spaces: vec![blank; size],
        }
    }

    fn valid_board_index(&self, index: usize) -> bool {
        index > 0 && index <= self.spaces.len()
    }
}

// This must be implemented in this file in order to access the private
// date inside the GameBoard stuct

impl <T: Clone + PartialEq> BasicBoard<T> for GameBoard<T>
{
    fn spaces(&self) -> &Vec<T> {
        &self.spaces
    }

    fn blank_value(&self) -> &T {
        &self.blank_value
    }

    fn set_space(&mut self, space: usize, value: T) {
        if self.valid_board_index(space) {
            self.spaces[space - 1] = value;
        }
    }

    fn get_space(&self, space: usize) -> &T {
            return &self.spaces[space - 1];
    }
}
