use super::ts;

pub use ts::Params;

pub struct Document {
    text: String,
    tree: ts::Tree,
}

impl Document {
    pub fn new(text: String, language: Language, params: Params) -> Self {
        let tree = ts::parse(&text, language, params);
        Self { text, tree }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tree(&self) -> &ts::Tree {
        &self.tree
    }

    pub fn walk(&self) -> Cursor {
        Cursor {
            doc: self,
            ts: self.tree.walk(),
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
    ts: ts::Cursor<'d>,
}

impl<'t> Cursor<'t> {
    pub fn node(&self) -> Node<'t> {
        Node {
            doc: self.doc,
            ts: self.ts.node(),
        }
    }

    pub fn goto_next_sibling(&mut self) -> bool {
        self.ts.goto_next_sibling()
    }

    pub fn goto_first_child(&mut self) -> bool {
        self.ts.goto_first_child()
    }

    pub fn goto_parent(&mut self) -> bool {
        self.ts.goto_parent()
    }
}

impl<'d> Deref for Cursor<'d> {
    type Target = ts::Cursor<'d>;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

#[derive(Clone)]
pub struct Node<'d> {
    doc: &'d Document,
    ts: ts::Node<'d>,
}

impl<'d> Node<'d> {
    pub fn text(&self) -> &'d str {
        &self.doc[self..=self]
    }

    pub fn walk(&self) -> Cursor<'d> {
        Cursor {
            doc: self.doc,
            ts: self.ts.walk(),
        }
    }
}

impl<'d> Deref for Node<'d> {
    type Target = ts::Node<'d>;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

pub use ts::Language;
