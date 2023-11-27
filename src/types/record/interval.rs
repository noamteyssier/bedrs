use crate::traits::{Coordinates, ValueBounds};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a classic Interval.
///
/// Has two coordinates: `start` and `end`.
///
/// ```
/// use bedrs::{Coordinates, Interval, Overlap};
///
/// let a = Interval::new(20, 30);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = Interval::new(25, 35);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Interval<T>
where
    T: ValueBounds,
{
    chr: T,
    start: T,
    end: T,
}
impl<T> Interval<T>
where
    T: ValueBounds,
{
    pub fn new(start: T, end: T) -> Self {
        Self {
            start,
            end,
            chr: T::default(),
        }
    }
}
impl<T> Coordinates<T, T> for Interval<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        Self {
            chr: zero::<T>(),
            start: zero::<T>(),
            end: zero::<T>(),
        }
    }
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
    #[allow(unused)]
    fn update_chr(&mut self, val: &T) {}
    fn from<Iv: Coordinates<T, T>>(other: &Iv) -> Self {
        Self {
            start: other.start(),
            end: other.end(),
            chr: T::default(),
        }
    }
}
impl<'a, T> Coordinates<T, T> for &'a Interval<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable reference to an empty interval")
    }
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
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}
impl<'a, T> Coordinates<T, T> for &'a mut Interval<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable reference to an empty interval")
    }
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
    #[allow(unused)]
    fn update_chr(&mut self, val: &T) {}
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
    }
}

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, types::Interval};
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let start = 10;
        let end = 100;
        let interval = Interval::new(start, end);

        assert_eq!(interval.start(), start);
        assert_eq!(interval.end(), end);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = Interval::new(10, 100);
        let b = Interval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = Interval::new(10, 100);
        let b = Interval::new(10, 90);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = Interval::new(5, 100);
        let b = Interval::new(10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = Interval::new(5, 90);
        let b = Interval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = Interval::new(5, 100);
        let b = Interval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn interval_serde() {
        let a = Interval::new(5, 100);
        let encoding = serialize(&a).unwrap();
        let b: Interval<usize> = deserialize(&encoding).unwrap();
        assert!(a.eq(&b));
    }

    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 0);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_none());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = Interval::new(10, 100);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
    }
}
