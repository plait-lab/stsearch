// Extended to: 1. minimize cursor cloning, 2. support sibling wildcards, & 3. track end

#[derive(Clone, Copy, Debug)]
pub enum Item<T> {
    Wildcard(Wildcard),
    Concrete(T),
}

#[derive(Clone, Copy, Debug)]
pub enum Wildcard {
    Subtree,
    Siblings,
}

pub fn match_at<T, C: Cursor<T>>(pattern: &[Item<T>], mut cursor: C) -> Option<C> {
    let mut checkpoints = vec![];

    const SKIP: Option<()> = Some(());
    let mut first = SKIP;
    let mut start = 0;

    'check: loop {
        'candidate: {
            for (i, token) in pattern.iter().enumerate().skip(start) {
                if let Item::Wildcard(Wildcard::Subtree) | Item::Concrete(_) = token {
                    if !(first.take().is_some() || cursor.move_next_subtree()) {
                        break 'candidate;
                    }
                }

                match token {
                    Item::Wildcard(Wildcard::Subtree) => {
                        checkpoints.push((Wildcard::Subtree, (i, None, cursor.clone())));
                    }
                    Item::Wildcard(Wildcard::Siblings) => {
                        let mut next = cursor.clone();
                        if first.is_some() || next.move_next_subtree() {
                            checkpoints.push((Wildcard::Subtree, (i, None, next.clone())));
                            checkpoints.push((Wildcard::Siblings, (i, SKIP, next)));
                        }
                    }
                    Item::Concrete(t) => {
                        let leaf = cursor.move_first_leaf();
                        if leaf != *t {
                            break 'candidate;
                        }
                    }
                }
            }
            return Some(cursor);
        }

        while let Some((kind, state)) = checkpoints.pop() {
            (start, first, cursor) = state;

            match kind {
                Wildcard::Subtree => {
                    assert!(first.is_none());
                    if cursor.move_first_child() {
                        first = SKIP; // no need to move
                        continue 'check;
                    }
                }
                Wildcard::Siblings => {
                    if first.take().is_some() || cursor.move_next_sibling() {
                        checkpoints.push((Wildcard::Siblings, (start, first, cursor.clone())));
                        start += 1; // skip wildcard
                        continue 'check;
                    }
                }
            }
        }

        return None;
    }
}

pub trait Cursor<T>: Clone {
    type Leaf: PartialEq<T>;

    fn move_first_leaf(&mut self) -> Self::Leaf;
    #[must_use]
    fn move_first_child(&mut self) -> bool;
    #[must_use]
    fn move_next_subtree(&mut self) -> bool;
    #[must_use]
    fn move_next_sibling(&mut self) -> bool;
}
