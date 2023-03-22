pub mod document;
pub mod mts;
pub mod ts;

pub trait Subtree {
    type Cursor: Traverse;
    type Node: Subtree;

    #[must_use]
    fn walk(self) -> Self::Cursor;
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
