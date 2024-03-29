use crate::{stmatch, Item, Pattern};

pub mod document;
pub mod lang;

use std::borrow::Borrow;

#[derive(Clone, Copy, Debug)]
pub struct Token<S: Borrow<str>>(S);

impl Token<String> {
    pub fn pattern(pattern: &str, language: lang::Select) -> Pattern<Self> {
        let (subtree, siblings) = match language {
            lang::Select::Javascript => ("$_", ("...", "/**/")),
        };

        // Ensure that siblings wildcard are parsed as extra
        let translated = pattern.replace(siblings.0, siblings.1);

        let mut tokens = vec![];
        document::Document::new(translated, language.parser())
            .walk()
            .foreach(|cursor| {
                let node = cursor.node();
                let range = node.byte_range();
                if node.child_count() == 0 && !range.is_empty() {
                    tokens.push(match cursor.text() {
                        token if token == subtree => Item::Subtree,
                        token if token == siblings.1 => Item::Siblings,
                        token => Item::Concrete(Token(token.to_owned())),
                    })
                }
            });

        Pattern(tokens)
    }
}

impl<'d> stmatch::Cursor<Token<String>> for document::Cursor<'d> {
    // Skips "extra" nodes to ignore comments when matching

    type Leaf = Token<&'d str>;

    fn move_first_leaf(&mut self) -> Self::Leaf {
        while self.move_first_child() {}
        Token(self.text())
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

impl<L: Borrow<str>, R: Borrow<str>> PartialEq<Token<R>> for Token<L> {
    fn eq(&self, other: &Token<R>) -> bool {
        self.0.borrow() == other.0.borrow()
    }
}
