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
    let mut phantom = std::iter::once(());
    let mut checkpoints = vec![];

    let mut pattern = pattern.iter();

    loop {
        loop {
            let pattern_c = pattern.clone();

            match pattern.next() {
                Some(token) => {
                    if !(phantom.next().is_some() || cursor.move_next_subtree()) {
                        break;
                    }

                    match token {
                        Item::Wildcard(wildcard) => {
                            checkpoints.push((
                                Wildcard::Subtree,
                                (pattern_c, cursor.clone(), phantom.clone()),
                            ));

                            if let Wildcard::Siblings = wildcard {
                                assert!(!phantom.next().is_some());
                                phantom = std::iter::once(());

                                checkpoints.push((
                                    Wildcard::Siblings,
                                    (pattern.clone(), cursor.clone(), phantom.clone()),
                                ));
                            }
                        }
                        Item::Concrete(t) => {
                            let leaf = cursor.move_first_leaf();
                            if leaf != *t {
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
            if let Some((kind, state)) = checkpoints.pop() {
                (pattern, cursor, phantom) = state;

                match kind {
                    Wildcard::Subtree => {
                        assert!(!phantom.next().is_some());
                        if cursor.move_first_child() {
                            phantom = std::iter::once(());
                            break;
                        }
                    }
                    Wildcard::Siblings => {
                        if phantom.next().is_some() || cursor.move_next_sibling() {
                            checkpoints.push((
                                Wildcard::Siblings,
                                (pattern.clone(), cursor.clone(), phantom.clone()),
                            ));
                            break;
                        }
                    }
                }
            } else {
                return None;
            }
        }
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
