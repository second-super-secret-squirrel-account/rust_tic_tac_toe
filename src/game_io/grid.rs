static GRID_TOP: &'static str = " -----";
static GRID_SECOND_START: &'static str = "|     ";
static GRID_SECOND_END: &'static str = "|";
static GRID_MIDDLE_START: &'static str = "|  ";
static GRID_MIDDLE_END: &'static str = "  ";

pub struct GridRow {
    row: Vec<char>,
}

impl GridRow {
    pub fn new(size: usize, fill: char) -> GridRow {
        let row = vec!(fill; size);
        GridRow {
            row: row,
        }
    }

    pub fn len(&self) -> usize {
       self.row.len()
    }

    pub fn get(&self, index: usize) -> char {
       self.row[index].clone()
    }

    pub fn set(&mut self, index: usize, value: char) {
        self.row[index] = value;
    }
}

pub struct Grid {
    grid: Vec<GridRow>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, fill: char) -> Grid {
        let mut grid = Vec::new();
        for index in 0..rows {
            let row = GridRow::new(cols, fill);
            grid.push(row);
        }
        Grid {
            grid: grid,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: char) {
        self.grid[row].set(col, value);
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.grid[row].get(col)
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }
 }
