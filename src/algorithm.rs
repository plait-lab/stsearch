use super::pattern;

pub fn match_at<'p, P, T, C>(mut pattern: P, mut cursor: C) -> Option<C>
where
    P: Iterator<Item = &'p pattern::Token<T>> + Checkpoint,
    C: Traverse + Checkpoint,
    T: 'p + PartialEq<C::Leaf>,
{
    impl CloneCheckpoint for std::iter::Once<()> {}

    let mut phantom = std::iter::once(());
    let mut checkpoints = vec![];

    loop {
        loop {
            let pattern_c = pattern.checkpoint();

            match pattern.next() {
                Some(token) => {
                    if !(phantom.next().is_some() || cursor.move_next_subtree()) {
                        break;
                    }

                    match token {
                        pattern::Token::Siblings => {
                            checkpoints.push((pattern_c, cursor.checkpoint(), phantom.checkpoint(), false));

                            assert!(!phantom.next().is_some());
                            phantom = std::iter::once(());

                            checkpoints.push((pattern.checkpoint(), cursor.checkpoint(), phantom.checkpoint(), true));
                        }
                        pattern::Token::Subtree => {
                            checkpoints.push((pattern_c, cursor.checkpoint(), phantom.checkpoint(), false));
                        }
                        pattern::Token::Leaf(t) => {
                            let leaf = cursor.move_first_leaf();
                            if *t != leaf {
                                break;
                            }
                        }
                    }
                }
                None => {
                    return Some(cursor);
                }
            };
        }
        loop {
            if let Some((pattern_c, cursor_c, phantom_c, siblings)) = checkpoints.pop() {
                pattern.restore(pattern_c);
                cursor.restore(cursor_c);
                phantom.restore(phantom_c);

                if siblings {
                    if phantom.next().is_some() || cursor.move_next_sibling() {
                        checkpoints.push((pattern.checkpoint(), cursor.checkpoint(), phantom.checkpoint(), true));
                        break;
                    }
                } else {
                    assert!(!phantom.next().is_some());
                    if cursor.move_first_child() {
                        phantom = std::iter::once(());
                        break;
                    }
                }
            } else {
                return None;
            }
        }
    }
}

// For backtracking algorithm
pub trait Checkpoint {
    type Checkpoint;

    #[must_use]
    fn checkpoint(&self) -> Self::Checkpoint;
    fn restore(&mut self, checkpoint: Self::Checkpoint);
}

pub trait CloneCheckpoint: Clone {}

impl<T: CloneCheckpoint> Checkpoint for T {
    type Checkpoint = T;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.clone()
    }

    fn restore(&mut self, checkpoint: Self::Checkpoint) {
        *self = checkpoint;
    }
}

// Inspired by tree_sitter::TreeCursor
pub trait Traverse {
    type Leaf;

    fn move_first_leaf(&mut self) -> Self::Leaf;
    #[must_use]
    fn move_first_child(&mut self) -> bool;
    #[must_use]
    fn move_next_subtree(&mut self) -> bool;
    #[must_use]
    fn move_next_sibling(&mut self) -> bool;
}

impl<C: crate::tree::Traverse> Traverse for C {
    type Leaf = C::Node;

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

    fn move_next_sibling(&mut self) -> bool {
        self.goto_next_sibling()
    }
}
