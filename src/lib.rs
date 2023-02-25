pub mod algorithm;
pub mod pattern;

pub mod tree;
pub use tree::document;

impl pattern::Pattern<String> {
    pub fn from_query(query: String, target: document::Language) -> Self {
        let document = document::Document::new(query, target, Default::default());

        use document::Traverse;

        document
            .walk()
            .leaves()
            .map(|node| {
                if node.kind() == "identifier" && node.text() == "$_" {
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
