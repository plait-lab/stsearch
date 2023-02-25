pub mod algorithm;
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

impl<'t> algorithm::Traverse for document::Cursor<'t> {
    type Leaf = document::Node<'t>;

    fn move_first_leaf(&mut self) -> Self::Leaf {
        while self.goto_first_child() {}
        self.node()
    }

    fn move_first_child(&mut self) -> bool {
        self.goto_first_child()
    }

    fn move_next_subtree(&mut self) -> bool {
        while !self.goto_next_sibling() {
            if !self.goto_parent() {
                return false;
            }
        }
        true
    }
}

impl PartialEq<document::Node<'_>> for String {
    fn eq(&self, other: &document::Node) -> bool {
        self == other.text()
    }
}

impl<'d> algorithm::CloneCheckpoint for document::Cursor<'d> {}
