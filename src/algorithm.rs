use super::pattern;

pub fn match_at<'p, P, T, C>(mut pattern: P, mut cursor: C) -> Option<C>
where
    P: Iterator<Item = &'p pattern::Token<T>> + Checkpoint,
    C: Traverse + Checkpoint,
    T: 'p + PartialEq<C::Leaf>,
{
    impl CloneCheckpoint for std::iter::Once<bool> {}

    let mut first = std::iter::once(true);
    let mut checkpoints = vec![];

    loop {
        loop {
            let pattern_c = pattern.checkpoint();

            match pattern.next() {
                Some(pattern) => {
                    if !(first.next().unwrap_or_default() || cursor.move_next_subtree()) {
                        break;
                    }

                    match pattern {
                        pattern::Token::Subtree => {
                            checkpoints.push((pattern_c, cursor.checkpoint(), first.checkpoint()));
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
            if let Some((pattern_c, cursor_c, first_c)) = checkpoints.pop() {
                pattern.restore(pattern_c);
                cursor.restore(cursor_c);
                first.restore(first_c);

                assert!(!first.next().unwrap_or_default());
                if cursor.move_first_child() {
                    first = std::iter::once(true);
                    break;
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
}
