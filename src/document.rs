use tree_sitter as ts;

pub struct Document {
    text: String,
    tree: ts::Tree,
}

impl Document {
    pub fn new(text: String, mut parser: ts::Parser) -> Self {
        let tree = parser
            .parse(&text, None)
            .expect("language is set, no timeout, and no cancel");

        Self { text, tree }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn walk(&self) -> Cursor {
        Cursor {
            text: &self.text,
            ts: self.tree.walk(),
        }
    }

    pub fn leaves(&self) -> Vec<&str> {
        let mut leaves = vec![];
        let mut cursor = self.walk();

        loop {
            while cursor.ts.goto_first_child() {}

            leaves.push(cursor.text());

            loop {
                if cursor.ts.goto_next_sibling() {
                    break;
                } else if cursor.ts.goto_parent() {
                    continue;
                }
                return leaves;
            }
        }
    }

    pub fn dim(&self) -> (usize, usize) {
        let (mut size, mut depth) = (1, 1);

        let mut cursor = self.walk();
        let mut level = 1;

        loop {
            while cursor.ts.goto_first_child() {
                size += 1;
                level += 1;
            }

            depth = std::cmp::max(depth, level);

            loop {
                if cursor.ts.goto_next_sibling() {
                    size += 1;
                    break;
                } else if cursor.ts.goto_parent() {
                    level -= 1;
                    continue;
                }
                assert_eq!(level, 1);
                return (size, depth);
            }
        }
    }
}

#[derive(Clone)]
pub struct Cursor<'d> {
    text: &'d str,
    pub ts: ts::TreeCursor<'d>,
}

impl<'d> Cursor<'d> {
    pub fn text(&self) -> &'d str {
        &self.text[self.ts.node().byte_range()]
    }
}
