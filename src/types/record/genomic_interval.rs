use crate::traits::{Coordinates, Overlap, ValueBounds};
use std::cmp::Ordering;

/// A representation of a Genomic Interval.
///
/// Has three coordinates: `chr`, `start`, and `end`.
///
/// ```
/// use bedrs::{Coordinates, GenomicInterval, Overlap};
///
/// let a = GenomicInterval::new(1, 20, 30);
/// assert_eq!(a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = GenomicInterval::new(1, 20, 30);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct GenomicInterval<T> {
    chr: T,
    start: T,
    end: T,
}

impl<T> Coordinates<T> for GenomicInterval<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> T {
        self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &T) {
        self.chr = *val;
    }
    fn from(other: &Self) -> Self {
        Self {
            chr: other.chr(),
            start: other.start(),
            end: other.end(),
        }
    }
}

impl<T> GenomicInterval<T>
where
    T: ValueBounds,
{
    pub fn new(chr: T, start: T, end: T) -> Self {
        Self { chr, start, end }
    }
}

impl<T> Ord for GenomicInterval<T>
where
    T: ValueBounds,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.chr().cmp(&other.chr()) {
            Ordering::Equal => self.start().cmp(&other.start()),
            order => order,
        }
    }
}

impl<T> PartialOrd for GenomicInterval<T>
where
    T: ValueBounds,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.chr().partial_cmp(&other.chr()) {
            None => None,
            Some(order) => match order {
                Ordering::Equal => self.start().partial_cmp(&other.start()),
                some_order => Some(some_order),
            },
        }
    }
}

impl<T> Overlap<T> for GenomicInterval<T> where T: ValueBounds {}

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, types::GenomicInterval};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = GenomicInterval::new(1, 10, 100);
        assert_eq!(interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = GenomicInterval::new(1, 10, 100);
        let b = GenomicInterval::new(1, 5, 100);
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = GenomicInterval::new(2, 10, 100);
        let b = GenomicInterval::new(1, 10, 100);
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = GenomicInterval::new(1, 5, 100);
        let b = GenomicInterval::new(1, 10, 100);
        assert_eq!(a.cmp(&b), Ordering::Less);

        let a = GenomicInterval::new(1, 10, 100);
        let b = GenomicInterval::new(2, 10, 100);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = GenomicInterval::new(1, 5, 100);
        let b = GenomicInterval::new(1, 5, 100);
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let a = GenomicInterval::new(1, 5, 100);
        let b = GenomicInterval::new(1, 5, 90);
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let a = GenomicInterval::new(2, 5, 100);
        let b = GenomicInterval::new(2, 5, 100);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }
}
