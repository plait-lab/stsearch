use super::algorithm::{match_at, Checkpoint, CloneCheckpoint, Traverse};

#[derive(Clone, Debug)]
pub struct Pattern<T> {
    sequence: std::vec::Vec<Token<T>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Token<T> {
    Siblings,
    Subtree,
    Leaf(T),
}

// Inspired by regex::Regex
impl<T> Pattern<T> {
    pub fn find<C>(&self, mut cursor: C) -> Option<Match<C>>
    where
        C: Traverse + Checkpoint + Clone,
        T: PartialEq<C::Leaf>,
    {
        let start = self
            .sequence
            .iter()
            // Skip any non-greedy Kleen stars
            .take_while(|t| matches!(t, Token::Siblings))
            .count();
        let sequence = &self.sequence[start..];
        loop {
            match Self::find_impl(sequence.iter(), cursor) {
                Ok(r#match) => return Some(r#match),
                Err(start) => cursor = start,
            }
            cursor.move_first_leaf();
            if !cursor.move_next_subtree() {
                break None;
            }
        }
    }

    pub fn find_iter<C>(&self, cursor: C) -> Matches<T, C>
    where
        C: Traverse + Checkpoint + Clone,
        T: PartialEq<C::Leaf>,
    {
        Matches {
            pattern: self,
            cursor: Some(cursor),
        }
    }

    pub fn find_at<C>(&self, start: C) -> Option<Match<C>>
    where
        C: Traverse + Checkpoint + Clone,
        T: PartialEq<C::Leaf>,
    {
        Self::find_impl(self.sequence.iter(), start).ok()
    }

    fn find_impl<'s, S, C>(mut seq: S, start: C) -> Result<Match<C>, C>
    where
        S: DoubleEndedIterator<Item = &'s Token<T>> + Checkpoint + Clone,
        C: Traverse + Checkpoint + Clone,
        T: 's + PartialEq<C::Leaf>,
    {
        loop {
            // FIX: Workaround, match_at includes an extra subtree otherwise
            let seq_c = seq.checkpoint();
            if !matches!(seq.next_back(), Some(Token::Siblings)) {
                seq.restore(seq_c);
                break;
            }
        }
        match match_at(seq.into_iter(), start.clone()) {
            Some(end) => Ok(Match { start, end }),
            None => Err(start),
        }
    }
}

impl<T> FromIterator<Token<T>> for Pattern<T> {
    fn from_iter<I: IntoIterator<Item = Token<T>>>(iter: I) -> Self {
        Self {
            sequence: iter.into_iter().collect(),
        }
    }
}

impl<'t, T> CloneCheckpoint for std::slice::Iter<'t, Token<T>> {}

// Inspired by regex::Matches
pub struct Matches<'p, T, C> {
    pattern: &'p Pattern<T>,
    cursor: Option<C>,
}

impl<'p, T, C> Iterator for Matches<'p, T, C>
where
    C: Traverse + Checkpoint + Clone,
    T: PartialEq<C::Leaf>,
{
    type Item = Match<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor
            .take()
            .and_then(|cursor| self.pattern.find(cursor))
            .map(|r#match| {
                let mut start = r#match.start.clone();
                if matches!(self.pattern.sequence.first(), Some(Token::Subtree)) {
                    // FIX: might cause duplicate matches
                    if start.move_first_child() || start.move_next_subtree() {
                        self.cursor = Some(start);
                    }
                } else {
                    start.move_first_leaf();
                    if start.move_next_subtree() {
                        self.cursor = Some(start);
                    }
                }
                r#match
            })
    }
}

// Inspired by regex::Match
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match<C> {
    pub start: C,
    pub end: C,
}
