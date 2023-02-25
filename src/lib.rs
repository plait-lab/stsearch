pub mod algorithm;
pub mod pattern;

pub mod tree;
pub use tree::document;

pub mod lang;

impl pattern::Pattern<String> {
    pub fn from_query(query: String, target: &lang::Language) -> Self {
        let pattern_language = lang::Select::Semgrep.load().nest("text", target);
        let document = document::Document::new(query, &pattern_language, Default::default());

        use document::Traverse;

        document
            .walk()
            .leaves()
            .map(|node| {
                if node.language() == *pattern_language
                    && (node.kind() == "metavar" || node.kind() == "ellipsis")
                {
                    pattern::Token::Subtree
                } else {
                    pattern::Token::Leaf(node.text().to_owned())
                }
            })
            .collect()
    }
}

impl PartialEq<document::Node<'_>> for String {
    fn eq(&self, other: &document::Node) -> bool {
        self == other.text()
    }
}

impl<'d> algorithm::CloneCheckpoint for document::Cursor<'d> {}
