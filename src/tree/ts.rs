use tree_sitter as ts;

pub use super::{Subtree, Traverse};

pub use ts::{Language, Node, Tree, TreeCursor as Cursor};

#[derive(Clone, Copy, Debug, Default)]
pub struct Params<'p> {
    pub ranges: Option<&'p [ts::Range]>,
}

pub fn parse(text: &str, language: Language, params: Params) -> Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .expect("version is compatible");
    if let Some(ranges) = params.ranges {
        parser
            .set_included_ranges(ranges)
            .expect("ranges meets req");
    }

    parser
        .parse(text, None)
        .expect("language is set, no timeout, and no cancel")
}

impl<'t> Subtree for &'t Tree {
    type Cursor = Cursor<'t>;
    type Node = Node<'t>;

    fn walk(self) -> Self::Cursor {
        self.walk()
    }
}

impl<'t> Traverse for Cursor<'t> {
    type Node = Node<'t>;

    fn node(&self) -> Self::Node {
        self.node()
    }

    fn goto_next_sibling(&mut self) -> bool {
        self.goto_next_sibling()
    }

    fn goto_first_child(&mut self) -> bool {
        self.goto_first_child()
    }

    fn goto_parent(&mut self) -> bool {
        self.goto_parent()
    }
}

impl<'t> Subtree for Node<'t> {
    type Cursor = Cursor<'t>;

    type Node = Self;

    fn walk(self) -> Self::Cursor {
        (&self).walk()
    }
}

impl super::document::ByteRange for Node<'_> {
    fn byte_range(&self) -> std::ops::Range<usize> {
        self.byte_range()
    }
}
