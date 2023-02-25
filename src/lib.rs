pub mod pattern;

pub mod tree;
pub use tree::document;

impl pattern::Pattern<String> {
    pub fn from_query(query: String, target: document::Language) -> Self {
        let document = document::Document::new(query, target, Default::default());

        let mut tokens = vec![];
        let mut cursor = document.walk();
        'traversal: loop {
            while cursor.goto_first_child() {}

            let node = cursor.node();
            tokens.push(if node.kind() == "identifier" && node.text() == "$_" {
                pattern::Token::Subtree
            } else {
                pattern::Token::Leaf(node.text().to_owned())
            });

            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break 'traversal;
                }
            }
        }

        tokens.drain(..).collect()
    }
}
