#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}
