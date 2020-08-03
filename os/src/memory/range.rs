#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range<T: From<usize> + Into<usize> + Copy> {
    pub start: T,
    pub end: T,
}

impl<T: From<usize> + Into<usize> + Copy, U: Into<T>> From<core::ops::Range<U>> for Range<T> {
    fn from(range: core::ops::Range<U>) -> Self {
        Self {
            // U into T
            start: range.start.into(),
            end: range.end.into()
        }
    }
}

impl<T: From<usize> + Into<usize> + Copy> Range<T> {
    pub fn overlap_with(&self, other: &Range<T>) -> bool {
        // T into usize
        self.start.into() < other.start.into() && self.end.into() > other.start.into()
    }
    pub fn iter(&self) -> impl Iterator<Item = T> {
        // T into usize, usize Iterator
        // map function: T::from<usize> means usize -> T
        (self.start.into()..self.end.into()).map(T::from)
    }
    pub fn len(&self) -> usize {
        // T into usize
        self.end.into() - self.start.into()
    }
    // mention: we use 'self' here to move the value
    pub fn into<U: From<usize> + Into<usize> + Copy + From<T>>(self) -> Range<U> {
        Range::<U> {
            // How do we know T -> U?
            // for U: From<T>
            start: U::from(self.start),
            end: U::from(self.end),
        }
    }
    pub fn get(&self, idx: usize) -> T {
        assert!(idx < self.len());
        // T -> usize, and then usize -> T
        T::from(self.start.into() + idx)
    }
    pub fn contains(&self, e: T) -> bool {
        e.into() >= self.start.into() && e.into() < self.end.into()
    }
}