pub use super::{Subtree, Traverse};

use std::borrow::Borrow;

pub struct Document<S, T>
where
    S: Borrow<str>,
    for<'t> &'t T: Subtree,
{
    text: S,
    tree: T,
}

pub fn new<S, T>(text: S, tree: T) -> Document<S, T>
where
    S: Borrow<str>,
    for<'t> &'t T: Subtree,
{
    Document::<S, T> { text, tree }
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
pub struct Cursor<'d, C>
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
pub struct Node<'d, N: Subtree> {
    text: &'d str,
    node: N,
}

impl<'d, N: Subtree + Deref> Node<'d, N>
where
    N::Target: ByteRange,
{
    pub fn text(&self) -> &'d str {
        &self.text[self.node.byte_range()]
    }
}

pub trait ByteRange {
    #[must_use]
    fn byte_range(&self) -> std::ops::Range<usize>;
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
