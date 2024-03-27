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
            path: vec![],
            ts: self.tree.walk(),
        }
    }

    pub fn leaves(&self) -> Vec<&str> {
        let mut leaves = vec![];
        self.walk().foreach(|cursor| {
            if cursor.node().child_count() == 0 {
                leaves.push(cursor.text())
            }
        });
        leaves
    }

    pub fn dim(&self) -> (usize, usize) {
        let (mut size, mut depth) = (1, 1);
        self.walk().foreach(|cursor| {
            depth = std::cmp::max(cursor.path.len() + 1, depth);
            size += 1;
        });
        return (size, depth);
    }
}

#[derive(Clone)]
pub struct Cursor<'d> {
    text: &'d str,
    path: Vec<usize>,
    ts: ts::TreeCursor<'d>,
}

impl<'d> Cursor<'d> {
    pub fn text(&self) -> &'d str {
        &self.text[self.ts.node().byte_range()]
    }

    pub fn path(&self) -> &[usize] {
        &self.path
    }

    pub fn node(&self) -> tree_sitter::Node {
        self.ts.node()
    }

    #[must_use]
    pub fn goto_first_child(&mut self) -> bool {
        self.ts
            .goto_first_child()
            .then(|| self.path.push(0))
            .is_some()
    }

    #[must_use]
    pub fn goto_next_sibling(&mut self) -> bool {
        self.path
            .last_mut()
            .and_then(|i| self.ts.goto_next_sibling().then(|| *i += 1))
            .is_some()
    }

    #[must_use]
    pub fn goto_parent(&mut self) -> bool {
        self.path
            .pop()
            .map(|_i| {
                assert!(self.ts.goto_parent());
            })
            .is_some()
    }

    #[must_use]
    pub fn goto_next_subtree(&mut self) -> bool {
        loop {
            if self.goto_next_sibling() {
                return true;
            } else if !self.goto_parent() {
                return false;
            }
        }
    }

    pub fn foreach<F: FnMut(&Cursor<'d>)>(&mut self, mut callback: F) {
        loop {
            callback(&self);
            while self.goto_first_child() {
                callback(&self);
            }

            if !self.goto_next_subtree() {
                break
            }
        }
    }
}
