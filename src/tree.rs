pub mod document;
pub mod mts;
pub mod ts;

pub trait Subtree {
    type Cursor: Traverse;
    type Node: Subtree;

    #[must_use]
    fn walk(self) -> Self::Cursor;

    #[must_use]
    fn dim(self) -> (usize, usize)
    where
        Self: Sized,
    {
        let (mut size, mut depth) = (1, 1);

        let mut cursor = self.walk();
        let mut level = 1;

        loop {
            while cursor.goto_first_child() {
                size += 1;
                level += 1;
            }

            depth = std::cmp::max(depth, level);

            loop {
                if cursor.goto_next_sibling() {
                    size += 1;
                    break;
                } else if cursor.goto_parent() {
                    level -= 1;
                    continue;
                }
                assert_eq!(level, 1);
                return  (size, depth)
            }
        }
    }
}

pub trait Traverse {
    type Node: Subtree;

    #[must_use]
    fn node(&self) -> Self::Node;

    #[must_use]
    fn goto_next_sibling(&mut self) -> bool;
    #[must_use]
    fn goto_first_child(&mut self) -> bool;
    #[must_use]
    fn goto_parent(&mut self) -> bool;

    #[must_use]
    fn leaves(self) -> Leaves<Self>
    where
        Self: Sized,
    {
        Leaves { cursor: Some(self) }
    }
}

#[derive(Clone)]
pub struct Leaves<Cursor: Traverse> {
    cursor: Option<Cursor>,
}

impl<Cursor: Traverse> Iterator for Leaves<Cursor> {
    type Item = Cursor::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.take().map(|mut cursor| {
            while cursor.goto_first_child() {}

            let leaf = cursor.node();

            loop {
                if cursor.goto_next_sibling() {
                    self.cursor = Some(cursor);
                } else if cursor.goto_parent() {
                    continue;
                }
                break;
            }

            leaf
        })
    }
}
