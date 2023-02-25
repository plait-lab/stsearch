pub mod document;
pub mod ts;

pub trait Traverse {
    type Node;

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

impl<C: Traverse> crate::algorithm::Traverse for C {
    type Leaf = C::Node;

    fn move_first_leaf(&mut self) -> Self::Leaf {
        while self.goto_first_child() {}
        self.node()
    }

    fn move_first_child(&mut self) -> bool {
        self.goto_first_child()
    }

    fn move_next_subtree(&mut self) -> bool {
        while !self.goto_next_sibling() {
            if !self.goto_parent() {
                return false;
            }
        }
        true
    }
}
