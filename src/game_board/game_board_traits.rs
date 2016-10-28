pub trait GameBoard<T> {
    fn get_space(&self, usize) -> T;
    fn set_space(&mut self, usize, T);
    fn corners(&self) -> [usize; 4];
}
