pub mod stmatch;

#[cfg(feature = "code")]
pub mod code;

#[derive(Clone, Debug)]
pub struct Pattern<T>(pub Vec<Item<T>>);
pub use stmatch::{Cursor, Item, Wildcard};

// Inspired by regex::Regex
impl<T> Pattern<T> {
    pub fn find<C: Cursor<T>>(&self, mut cursor: C) -> Option<Match<C>> {
        let start = self
            .0
            .iter()
            .take_while(|t| matches!(t, Item::Wildcard(Wildcard::Siblings)))
            .count();
        let sequence = &self.0[start..];
        loop {
            match Self::find_impl(sequence, cursor) {
                Ok(r#match) => return Some(r#match),
                Err(start) => cursor = start,
            }
            cursor.move_first_leaf();
            if !cursor.move_next_subtree() {
                break None;
            }
        }
    }

    pub fn find_iter<C: Cursor<T>>(&self, cursor: C) -> Matches<T, C> {
        Matches {
            pattern: self,
            cursor: Some(cursor),
        }
    }

    pub fn find_at<C: Cursor<T>>(&self, start: C) -> Option<Match<C>> {
        Self::find_impl(&self.0, start).ok()
    }

    fn find_impl<C: Cursor<T>>(sequence: &[Item<T>], start: C) -> Result<Match<C>, C> {
        match stmatch::match_at(sequence, start.clone()) {
            Some(end) => Ok(Match { start, end }),
            None => Err(start),
        }
    }

    pub fn holes(&self) -> usize {
        self.0
            .iter()
            .filter(|t| matches!(t, Item::Wildcard(_)))
            .count()
    }
}

impl<T> FromIterator<Item<T>> for Pattern<T> {
    fn from_iter<I: IntoIterator<Item = Item<T>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

// Inspired by regex::Matches
pub struct Matches<'p, T, C: Cursor<T>> {
    pattern: &'p Pattern<T>,
    cursor: Option<C>,
}

impl<'p, T, C: Cursor<T>> Iterator for Matches<'p, T, C> {
    type Item = Match<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor
            .take()
            .and_then(|cursor| self.pattern.find(cursor))
            .map(|r#match| {
                let mut start = r#match.start.clone();
                match self.pattern.0.first() {
                    Some(Item::Wildcard(_)) | None => {
                        if start.move_first_child() || start.move_next_subtree() {
                            self.cursor = Some(start);
                        }
                    }
                    Some(Item::Concrete(_)) => {
                        start.move_first_leaf();
                        if start.move_next_subtree() {
                            self.cursor = Some(start);
                        }
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
