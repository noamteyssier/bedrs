use std::cmp::Ordering;

use crate::traits::{Coordinates, GenomicCoordinates, GenomicOverlap};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GenomicInterval<T> {
    chr: T,
    start: T,
    end: T,
}

impl<T> Coordinates<T> for GenomicInterval<T> {
    fn start(&self) -> &T {
        &self.start
    }
    fn end(&self) -> &T {
        &self.end
    }
}

impl<T> GenomicCoordinates<T> for GenomicInterval<T> {
    fn chr(&self) -> &T {
        &self.chr
    }
}
impl<T: PartialOrd> GenomicOverlap<T> for GenomicInterval<T> {}

impl<T> GenomicInterval<T>
where
    T: Copy,
{
    pub fn new(chr: T, start: T, end: T) -> Self {
        Self { chr, start, end }
    }
    pub fn from<I: GenomicCoordinates<T>>(other: &I) -> Self {
        Self {
            chr: *other.chr(),
            start: *other.start(),
            end: *other.end(),
        }
    }
    pub fn update_start(&mut self, value: &T) {
        self.start = *value;
    }
    pub fn update_end(&mut self, value: &T) {
        self.end = *value;
    }
}

impl<T> Ord for GenomicInterval<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.chr().cmp(other.chr()) {
            Ordering::Equal => self.start().cmp(other.start()),
            order => order,
        }
    }
}

impl<T> PartialOrd for GenomicInterval<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.chr().partial_cmp(other.chr()) {
            None => None,
            Some(order) => match order {
                Ordering::Equal => self.start().partial_cmp(other.start()),
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
        assert_eq!(interval.chr(), &1);
        assert_eq!(interval.start(), &10);
        assert_eq!(interval.end(), &100);
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
