use crate::traits::{Coordinates, ValueBounds};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Genomic Interval.
///
/// Has three coordinates: `chr`, `start`, and `end`.
///
/// ```
/// use bedrs::{Coordinates, GenomicInterval, Overlap};
///
/// let a = GenomicInterval::new(1, 20, 30);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = GenomicInterval::new(1, 20, 30);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenomicInterval<T> {
    chr: T,
    start: T,
    end: T,
}

impl<T> Coordinates<T, T> for GenomicInterval<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &T {
        &self.chr
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
            chr: *other.chr(),
            start: other.start(),
            end: other.end(),
        }
    }
}
impl<'a, T> Coordinates<T, T> for &'a GenomicInterval<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &T {
        &self.chr
    }
    #[allow(unused)]
    fn update_start(&mut self, val: &T) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn update_end(&mut self, val: &T) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn update_chr(&mut self, val: &T) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn from(other: &Self) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}
impl<'a, T> Coordinates<T, T> for &'a mut GenomicInterval<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &T {
        &self.chr
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
    #[allow(unused)]
    fn from(other: &Self) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
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

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, types::GenomicInterval};
    use std::cmp::Ordering;

    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    #[test]
    fn test_interval_init() {
        let interval = GenomicInterval::new(1, 10, 100);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = GenomicInterval::new(1, 10, 100);
        let b = GenomicInterval::new(1, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = GenomicInterval::new(1, 10, 100);
        let b = GenomicInterval::new(1, 10, 90);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = GenomicInterval::new(2, 10, 100);
        let b = GenomicInterval::new(1, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = GenomicInterval::new(1, 5, 100);
        let b = GenomicInterval::new(1, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = GenomicInterval::new(1, 10, 100);
        let b = GenomicInterval::new(2, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = GenomicInterval::new(1, 5, 100);
        let b = GenomicInterval::new(1, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = GenomicInterval::new(2, 5, 100);
        let b = GenomicInterval::new(2, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn genomic_interval_serde() {
        let a = GenomicInterval::new(1, 5, 100);
        let encoding = serialize(&a).unwrap();
        let b: GenomicInterval<usize> = deserialize(&encoding).unwrap();
        assert_eq!(a, b);
    }

    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_none());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = GenomicInterval::new(1, 10, 100);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
    }
}
