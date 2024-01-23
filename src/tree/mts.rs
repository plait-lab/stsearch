// Hierarchical [M]ulti-language tree_sitter support

use super::ts;

use std::ops::Deref;

pub use super::{Subtree, Traverse};

pub use ts::Params;

#[derive(Debug)]
pub struct Tree {
    ts: ts::Tree,
    nested: Vec<(Range, Tree)>,
}

impl Tree {
    pub fn new(text: &str, language: &Language, params: Params) -> Self {
        Self::ranged_new(text, language, params)
    }

    fn ranged_new(text: &str, language: &Language, params: Params) -> Self {
        let ts = ts::parse(text, **language, params);

        let nested = if language.nested.is_empty() {
            vec![] // skip traversing the leaves
        } else {
            ts.walk()
                .leaves()
                .filter_map(|leaf| {
                    language.get(leaf.kind_id()).map(|language| {
                        let ranges = [leaf.range()];
                        let params = Params {
                            // TODO: intersect with params.ranges
                            ranges: Some(&ranges),
                        };
                        let tree = Tree::ranged_new(text, language, params);
                        (leaf.byte_range(), tree)
                    })
                })
                .collect()
        };

        Self { ts, nested }
    }

    fn nested_find(&self, node: &Node) -> Option<&Tree> {
        use std::cmp::Ordering;

        let range = node.byte_range();
        self.nested
            .binary_search_by(|(curr, _)| {
                if curr.end <= range.start {
                    Ordering::Less
                } else if range.end <= curr.start {
                    Ordering::Greater
                } else if range.start <= curr.start && curr.end <= range.end {
                    Ordering::Equal
                } else {
                    panic!("ranges shouldn't overlap")
                }
            })
            .map(|i| &self.nested[i].1)
            .ok()
    }
}

impl<'t> Subtree for &'t Tree {
    type Cursor = Cursor<'t>;
    type Node = Node<'t>;

    fn walk(self) -> Self::Cursor {
        Cursor {
            parent: None,
            current: self,
            ts: self.ts.walk(),
        }
    }
}

impl Deref for Tree {
    type Target = ts::Tree;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

#[derive(Clone)]
pub struct Cursor<'t> {
    parent: Parent<'t>,
    current: &'t Tree,
    ts: ts::Cursor<'t>,
}

impl<'t> Traverse for Cursor<'t> {
    type Node = Node<'t>;

    fn node(&self) -> Node<'t> {
        Node {
            tree: self.current,
            ts: self.ts.node(),
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
        Traverse::goto_next_sibling(&mut self.ts)
    }

    fn goto_first_child(&mut self) -> bool {
        Traverse::goto_first_child(&mut self.ts) || {
            let node = self.node();
            self.current
                .nested_find(&node)
                .map(|tree| {
                    let mut other = Self {
                        parent: Default::default(), // dummy
                        current: tree,
                        ts: tree.ts.walk(),
                    };
                    std::mem::swap(self, &mut other);
                    self.parent = other.into();
                })
                .is_some()
        }
    }

    fn goto_parent(&mut self) -> bool {
        self.ts.goto_parent() || {
            self.parent
                .take()
                .try_into()
                .map(|cursor| *self = cursor)
                .is_ok()
        }
    }
}

impl<'t> Deref for Cursor<'t> {
    type Target = ts::Cursor<'t>;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node<'t> {
    tree: &'t Tree,
    ts: ts::Node<'t>,
}

impl<'t> Node<'t> {
    pub fn language(&self) -> ts::Language {
        self.tree.language()
    }
}

impl<'t> Subtree for Node<'t> {
    type Cursor = Cursor<'t>;
    type Node = Node<'t>;

    fn walk(self) -> Cursor<'t> {
        Cursor {
            parent: None,
            current: self.tree,
            ts: self.ts.walk(),
        }
    }
}

impl<'t> Deref for Node<'t> {
    type Target = ts::Node<'t>;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

use std::rc::Rc;

type Parent<'t> = Option<Rc<Cursor<'t>>>;

impl<'t> TryFrom<Parent<'t>> for Cursor<'t> {
    type Error = &'static str;

    fn try_from(parent: Parent<'t>) -> Result<Self, Self::Error> {
        parent
            .map(|p| Rc::try_unwrap(p).unwrap_or_else(|p| (*p).clone()))
            .ok_or("already at root")
    }
}

impl<'t> From<Cursor<'t>> for Parent<'t> {
    fn from(cursor: Cursor<'t>) -> Self {
        Some(Rc::new(cursor))
    }
}

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Language<'l> {
    ts: ts::Language,
    nested: HashMap<u16, &'l Language<'l>>,
}

impl<'l> Language<'l> {
    pub fn nest(mut self, node_kind: &'static str, language: &'l Language) -> Self {
        let id = self.id_for_node_kind(node_kind, true);
        self.nested.insert(id, language);
        self
    }

    fn get(&self, kind_id: u16) -> Option<&Self> {
        self.nested.get(&kind_id).copied()
    }
}

impl From<ts::Language> for Language<'_> {
    fn from(language: ts::Language) -> Self {
        let nested = HashMap::new();
        Self {
            ts: language,
            nested,
        }
    }
}

impl Deref for Language<'_> {
    type Target = ts::Language;

    fn deref(&self) -> &Self::Target {
        &self.ts
    }
}

type Range = std::ops::Range<usize>;
