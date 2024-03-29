#[derive(Clone, Copy, Debug)]
pub enum Item<T> {
    Subtree,
    Siblings,
    Concrete(T),
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
                        Item::Siblings => {
                            checkpoints.push((pattern_c, cursor.clone(), phantom.clone(), false));

                            assert!(!phantom.next().is_some());
                            phantom = std::iter::once(());

                            checkpoints.push((
                                pattern.clone(),
                                cursor.clone(),
                                phantom.clone(),
                                true,
                            ));
                        }
                        Item::Subtree => {
                            checkpoints.push((pattern_c, cursor.clone(), phantom.clone(), false));
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
            if let Some(checkpoint) = checkpoints.pop() {
                let siblings;
                (pattern, cursor, phantom, siblings) = checkpoint;

                if siblings {
                    if phantom.next().is_some() || cursor.move_next_sibling() {
                        checkpoints.push((pattern.clone(), cursor.clone(), phantom.clone(), true));
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
