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

    pub fn dim(&self) -> (usize, usize) {
        let (mut size, mut depth) = (0, 0);
        self.walk().foreach(|path, _| {
            depth = std::cmp::max(path.len() + 1, depth);
            size += 1;
        });
        return (size, depth);
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

    pub fn foreach<F: FnMut(&Path, &Cursor<'d>)>(&mut self, mut callback: F) {
        let mut path = vec![];
        loop {
            callback(&path, &self);
            if self.ts.goto_first_child() {
                path.push(0);
                continue;
            }
            loop {
                if self.ts.goto_next_sibling() {
                    *path.last_mut().unwrap() += 1;
                    break;
                } else if self.ts.goto_parent() {
                    path.pop();
                    continue;
                }
                return;
            }
        }
    }
}

type Path = [usize];
