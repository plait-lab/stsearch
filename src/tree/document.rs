use super::mts;

pub use super::{Subtree, Traverse};

pub use mts::Params;

pub struct Document {
    text: String,
    tree: mts::Tree,
}

impl Document {
    pub fn new(text: String, language: &Language, params: Params) -> Self {
        let tree = mts::Tree::new(&text, language, params);
        Self { text, tree }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tree(&self) -> &mts::Tree {
        &self.tree
    }
}

impl<'d> Subtree for &'d Document {
    type Cursor = Cursor<'d>;
    type Node = Node<'d>;

    fn walk(self) -> Cursor<'d> {
        Cursor {
            doc: self,
            mts: self.tree.walk(),
        }
    }
}

use std::ops::{Index, RangeInclusive};

impl<'d, N> Index<RangeInclusive<N>> for Document
where
    N: std::borrow::Borrow<Node<'d>>,
{
    type Output = str;

    fn index(&self, index: RangeInclusive<N>) -> &Self::Output {
        &self.text[index.start().borrow().start_byte()..index.end().borrow().end_byte()]
    }
}

use std::ops::Deref;

#[derive(Clone)]
pub struct Cursor<'d> {
    doc: &'d Document,
    mts: mts::Cursor<'d>,
}

impl<'d> Traverse for Cursor<'d> {
    type Node = Node<'d>;

    fn node(&self) -> Self::Node {
        Node {
            doc: self.doc,
            mts: self.mts.node(),
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
        self.mts.goto_next_sibling()
    }

    fn goto_first_child(&mut self) -> bool {
        self.mts.goto_first_child()
    }

    fn goto_parent(&mut self) -> bool {
        self.mts.goto_parent()
    }
}

impl<'d> Deref for Cursor<'d> {
    type Target = mts::Cursor<'d>;

    fn deref(&self) -> &Self::Target {
        &self.mts
    }
}

#[derive(Clone)]
pub struct Node<'d> {
    doc: &'d Document,
    mts: mts::Node<'d>,
}

impl<'d> Node<'d> {
    pub fn text(&self) -> &'d str {
        &self.doc[self..=self]
    }
}

impl<'d> Subtree for Node<'d> {
    type Cursor = Cursor<'d>;
    type Node = Node<'d>;

    fn walk(self) -> Cursor<'d> {
        Cursor {
            doc: self.doc,
            mts: self.mts.walk(),
        }
    }
}

impl<'d> Deref for Node<'d> {
    type Target = mts::Node<'d>;

    fn deref(&self) -> &Self::Target {
        &self.mts
    }
}

pub use mts::Language;
