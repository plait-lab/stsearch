use crate::Token;

pub fn match_at<'p, P, T, C>(mut pattern: P, mut cursor: C) -> Option<C>
where
    P: Iterator<Item = &'p Token<T>> + Clone,
    C: Traverse,
    T: 'p + PartialEq<C::Leaf>,
{
    let mut phantom = std::iter::once(());
    let mut checkpoints = vec![];

    loop {
        loop {
            let pattern_c = pattern.clone();

            match pattern.next() {
                Some(token) => {
                    if !(phantom.next().is_some() || cursor.move_next_subtree()) {
                        break;
                    }

                    match token {
                        Token::Siblings => {
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
                        Token::Subtree => {
                            checkpoints.push((pattern_c, cursor.clone(), phantom.clone(), false));
                        }
                        Token::Leaf(t) => {
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


pub trait Traverse: Clone {
    type Leaf;

    fn move_first_leaf(&mut self) -> Self::Leaf;
    #[must_use]
    fn move_first_child(&mut self) -> bool;
    #[must_use]
    fn move_next_subtree(&mut self) -> bool;
    #[must_use]
    fn move_next_sibling(&mut self) -> bool;
}
