#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cursor {
    row: i32,
    col: i32,
    row_size: i32,
    col_size: i32,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            row_size: 2,
            col_size: 2,
            row: 1,
            col: 1,
        }
    }

    pub fn index(&self) -> usize {
        (self.row * (1 + self.row_size) + self.col) as usize
    }

    pub fn set_space(&mut self, space: usize) {
        let space = space as i32;
        self.row = space / (self.row_size + 1);
        self.col = space % (self.row_size + 1);
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    pub fn move_left(&mut self) {
        if self.col != 0 {
            self.col = self.col - 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.col != self.col_size {
            self.col = self.col + 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.row != 0 {
            self.row = self.row - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.row != self.row_size {
            self.row = self.row + 1;
        }
    }
}
