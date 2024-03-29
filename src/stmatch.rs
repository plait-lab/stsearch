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

    let mut phantom = std::iter::once(());
    let mut start = 0;

    'check: loop {
        'candidate: {
            for (i, token) in pattern.iter().enumerate().skip(start) {
                if !(phantom.next().is_some() || cursor.move_next_subtree()) {
                    break 'candidate;
                }

                match token {
                    Item::Wildcard(wildcard) => {
                        checkpoints.push((
                            Wildcard::Subtree, // might be further down
                            (i, cursor.clone(), phantom.clone()),
                        ));

                        if let Wildcard::Siblings = wildcard {
                            assert!(!phantom.next().is_some());
                            phantom = std::iter::once(());

                            checkpoints.push((
                                Wildcard::Siblings, // might include more node
                                (i, cursor.clone(), phantom.clone()),
                            ));
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
            (start, cursor, phantom) = state;

            match kind {
                Wildcard::Subtree => {
                    assert!(!phantom.next().is_some());
                    if cursor.move_first_child() {
                        phantom = std::iter::once(());
                        continue 'check;
                    }
                }
                Wildcard::Siblings => {
                    if phantom.next().is_some() || cursor.move_next_sibling() {
                        checkpoints.push((
                            Wildcard::Siblings, // might be on left spine
                            (start, cursor.clone(), phantom.clone()),
                        ));
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
