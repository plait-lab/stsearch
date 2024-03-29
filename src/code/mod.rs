use crate::{stmatch, Pattern, Item};

pub mod lang;
pub mod document;

impl Pattern<String> {
    pub fn from_query(mut query: String, language: lang::Select) -> Self {
        let (subtree, siblings) = match language {
            lang::Select::Javascript => ("$_", ("...", "/**/")),
        };

        // Ensure that siblings wildcard are parsed as extra
        query = query.replace(siblings.0, siblings.1);

        let document = document::Document::new(query, language.parser());

        document
            .leaves()
            .drain(..)
            .flat_map(|leaf| match leaf {
                "" => None,
                tok if tok == subtree => Some(Item::Subtree),
                tok if tok == siblings.1 => Some(Item::Siblings),
                leaf => Some(Item::Concrete(leaf.to_owned())),
            })
            .collect()
    }
}

impl<'d> stmatch::Cursor<String> for document::Cursor<'d> {
    // Skips "extra" nodes to effectively drop comments

    type Leaf = &'d str;

    fn move_first_leaf(&mut self) -> Self::Leaf {
        while self.move_first_child() {}
        self.text()
    }

    fn move_first_child(&mut self) -> bool {
        self.goto_first_child()
            && (!self.node().is_extra() || self.move_next_sibling() || !self.goto_parent())
    }

    fn move_next_subtree(&mut self) -> bool {
        while !self.move_next_sibling() {
            if !self.goto_parent() {
                return false;
            }
        }
        true
    }

    fn move_next_sibling(&mut self) -> bool {
        while self.goto_next_sibling() {
            if !self.node().is_extra() {
                return true;
            }
        }
        return false;
    }
}
