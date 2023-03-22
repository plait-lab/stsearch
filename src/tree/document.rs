pub use super::{Subtree, Traverse};

use std::borrow::Borrow;

use super::mts; // default tree

pub struct Document<S = String, T = mts::Tree>
where
    S: Borrow<str>,
    for<'t> &'t T: Subtree,
{
    text: S,
    tree: T,
}

impl<S, T> Document<S, T>
where
    S: Borrow<str>,
    for<'t> &'t T: Subtree,
{
    pub fn text(&self) -> &str {
        self.text.borrow()
    }

    pub fn tree(&self) -> &T {
        &self.tree
    }
}

impl<'d, S, T> Subtree for &'d Document<S, T>
where
    S: Borrow<str>,
    for<'t> &'t T: Subtree,
{
    type Cursor = Cursor<'d, <&'d T as Subtree>::Cursor>;
    type Node = Node<'d, <&'d T as Subtree>::Node>;

    fn walk(self) -> Self::Cursor {
        Cursor {
            text: self.text.borrow(),
            cursor: self.tree.walk(),
        }
    }
}

use std::ops::Deref;

#[derive(Clone)]
pub struct Cursor<'d, C = mts::Cursor<'d>>
where
    C: Traverse,
{
    text: &'d str,
    cursor: C,
}

impl<'d, C> Traverse for Cursor<'d, C>
where
    C: Traverse,
{
    type Node = Node<'d, C::Node>;

    fn node(&self) -> Self::Node {
        Node {
            text: self.text,
            node: self.cursor.node(),
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
        self.cursor.goto_next_sibling()
    }

    fn goto_first_child(&mut self) -> bool {
        self.cursor.goto_first_child()
    }

    fn goto_parent(&mut self) -> bool {
        self.cursor.goto_parent()
    }
}

impl<'d, C> Deref for Cursor<'d, C>
where
    C: Traverse,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

#[derive(Clone, Copy)]
pub struct Node<'d, N: Subtree = mts::Node<'d>> {
    text: &'d str,
    node: N,
}

use std::ops::Index;

impl<'d, N: Subtree> Node<'d, N>
where
    for<'n> str: Index<&'n N>,
{
    pub fn text(&self) -> &'d <str as Index<&'_ N>>::Output {
        &self.text[&self.node]
    }
}

impl<'d, N: Subtree> Subtree for Node<'d, N> {
    type Cursor = Cursor<'d, N::Cursor>;
    type Node = Self;

    fn walk(self) -> Self::Cursor {
        Cursor {
            text: self.text,
            cursor: self.node.walk(),
        }
    }
}

impl<'d, N: Subtree> Deref for Node<'d, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<S> Document<S, mts::Tree>
where
    S: Borrow<str>,
{
    pub fn new(text: S, language: &mts::Language, params: mts::Params) -> Self {
        let tree = mts::Tree::new(text.borrow(), language, params);
        Self { text, tree }
    }
}

impl<'n> Index<&'n mts::Node<'_>> for str {
    type Output = str;

    fn index(&self, index: &'n mts::Node) -> &Self::Output {
        &self[index.byte_range()]
    }
}
