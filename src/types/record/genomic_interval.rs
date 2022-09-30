use std::cmp::Ordering;

use crate::traits::{Coordinates, GenomicCoordinates, GenomicOverlap};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct GenomicInterval<T> {
    chr: T,
    start: T,
    end: T,
}

impl<T> Coordinates<T> for GenomicInterval<T>
where
    T: Copy + Default,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
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
    T: Copy + Default,
{
    pub fn new(chr: T, start: T, end: T) -> Self {
        Self { chr, start, end }
    }
}

impl<T> Ord for GenomicInterval<T>
where
    T: Eq + Ord + Copy + Default,
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
    T: Ord + Copy + Default,
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

#[cfg(test)]
mod testing {
    use crate::{
        traits::{Coordinates, GenomicCoordinates},
        types::GenomicInterval,
    };
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
