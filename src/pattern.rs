#[derive(Clone, Debug)]
pub struct Pattern<T> {
    sequence: std::vec::Vec<Token<T>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Token<T> {
    Subtree,
    Leaf(T),
}

impl<T> FromIterator<Token<T>> for Pattern<T> {
    fn from_iter<I: IntoIterator<Item = Token<T>>>(iter: I) -> Self {
        Self {
            sequence: iter.into_iter().collect(),
        }
    }
}
