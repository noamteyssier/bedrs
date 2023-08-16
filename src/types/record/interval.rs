use crate::traits::{Coordinates, ValueBounds};
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
    start: T,
    end: T,
}
impl<T> Interval<T>
where
    T: ValueBounds,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}
impl<T> Coordinates<T, T> for Interval<T>
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
        T::default()
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    #[allow(unused)]
    fn update_chr(&mut self, val: &T) {}
    fn from(other: &Self) -> Self {
        Self {
            start: other.start(),
            end: other.end(),
        }
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
    fn test_serialization() {
        let a = Interval::new(5, 100);
        let encoding = serialize(&a).unwrap();
        let b: Interval<usize> = deserialize(&encoding).unwrap();
        assert!(a.eq(&b));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_deserialization() {
        let encoding = vec![5, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0];
        let expected = Interval::new(5, 100);
        let observed: Interval<usize> = deserialize(&encoding).unwrap();
        assert!(expected.eq(&observed));
    }
}
