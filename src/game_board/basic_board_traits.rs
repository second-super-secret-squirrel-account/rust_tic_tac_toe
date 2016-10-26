pub trait BasicBoard<T> {
    fn spaces(&self) -> &Vec<T>;
    fn blank_value(&self) -> &T;
    fn get_space(&self, usize) -> &T;
    fn set_space(&mut self, usize, T);
}
