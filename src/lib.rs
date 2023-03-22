pub mod algorithm;
pub mod pattern;

pub mod tree;
pub use tree::document;

pub mod lang;

use tree::mts::{Cursor, Node, Tree};

impl pattern::Pattern<String> {
    pub fn from_query(query: String, target: &lang::Language) -> Self {
        let pattern_language = lang::Select::Semgrep.load().nest("text", target);
        let tree = Tree::new(&query, &pattern_language, Default::default());
        let document = document::new::<String, Tree>(query, tree);

        use document::{Subtree, Traverse};

        document
            .walk()
            .leaves()
            .flat_map(|node| {
                if node.language() == *pattern_language && node.kind() == "ellipsis" {
                    Some(pattern::Token::Siblings)
                } else if node.language() == *pattern_language && node.kind() == "metavar" {
                    Some(pattern::Token::Subtree)
                } else if !node.text().is_empty() {
                    Some(pattern::Token::Leaf(node.text().to_owned()))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl PartialEq<document::Node<'_, Node<'_>>> for String {
    fn eq(&self, other: &document::Node<Node>) -> bool {
        self == other.text()
    }
}

impl algorithm::CloneCheckpoint for document::Cursor<'_, Cursor<'_>> {}
