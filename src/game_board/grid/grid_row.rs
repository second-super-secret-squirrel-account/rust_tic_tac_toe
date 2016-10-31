#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GridRow<T> {
    row: Vec<T>,
}

impl <T: Clone + PartialEq> GridRow<T> {
    pub fn new(size: usize, fill: T) -> GridRow<T> {
        let row = vec!(fill; size);
        GridRow {
            row: row,
        }
    }

    pub fn len(&self) -> usize {
       self.row.len()
    }

    pub fn get(&self, index: usize) -> T {
       self.row[index].clone()
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.row[index] = value;
    }
}
