#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Clone + PartialEq> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y,
        }
    }
}
