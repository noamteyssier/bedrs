use crate::traits::{Coordinates, ValueBounds};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a classic Interval.
///
/// Has two coordinates: `start` and `end`.
///
/// ```
/// use bedrs::{Coordinates, BaseInterval, Overlap};
///
/// let a = BaseInterval::new(20, 30);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = BaseInterval::new(25, 35);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseInterval<T>
where
    T: ValueBounds,
{
    #[cfg_attr(feature = "serde", serde(skip))]
    chr: T,
    start: T,
    end: T,
}
impl<T> BaseInterval<T>
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
impl<T> Coordinates<T, T> for BaseInterval<T>
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
impl<'a, T> Coordinates<T, T> for &'a BaseInterval<T>
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
impl<'a, T> Coordinates<T, T> for &'a mut BaseInterval<T>
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
    use crate::{traits::Coordinates, types::BaseInterval};
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let start = 10;
        let end = 100;
        let iv = BaseInterval::new(start, end);

        assert_eq!(iv.start(), start);
        assert_eq!(iv.end(), end);
        assert_eq!(
            format!("{iv:?}"),
            "BaseInterval { chr: 0, start: 10, end: 100 }"
        );
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = BaseInterval::new(10, 100);
        let b = BaseInterval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = BaseInterval::new(10, 100);
        let b = BaseInterval::new(10, 90);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = BaseInterval::new(5, 100);
        let b = BaseInterval::new(10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = BaseInterval::new(5, 90);
        let b = BaseInterval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = BaseInterval::new(5, 100);
        let b = BaseInterval::new(5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn interval_serde() {
        let a = BaseInterval::new(5, 100);
        let encoding = serialize(&a).unwrap();
        let b: BaseInterval<usize> = deserialize(&encoding).unwrap();
        assert!(a.eq(&b));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 0);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_none());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = BaseInterval::new(10, 100);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_testing {
    use super::*;
    use anyhow::Result;
    use csv::WriterBuilder;

    #[test]
    fn test_csv_serialization() -> Result<()> {
        let a = BaseInterval::new(20, 30);
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "20,30\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "20,30\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: BaseInterval<i32> = iter.next().unwrap()?;
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        Ok(())
    }
}
