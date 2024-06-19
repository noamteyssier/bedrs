use crate::traits::Coordinates;
use derive_new::new;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a classic Interval.
///
/// Has two coordinates: `start` and `end`.
///
/// ```
/// use bedrs::prelude::*;
///
/// let a = BaseInterval::new(20, 30);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = BaseInterval::new(25, 35);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy, Default, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseInterval {
    start: i32,
    end: i32,
}
impl Coordinates<i32> for BaseInterval {
    fn chr(&self) -> &i32 {
        &0
    }
    fn start(&self) -> i32 {
        self.start
    }
    fn end(&self) -> i32 {
        self.end
    }
    fn update_start(&mut self, val: &i32) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &i32) {
        self.end = *val;
    }
    fn update_chr(&mut self, _val: &i32) {
        // Do nothing
    }
    fn empty() -> Self {
        Self::default()
    }
    fn from<Iv: Coordinates<i32>>(other: &Iv) -> Self {
        Self::new(other.start(), other.end())
    }
}
impl Coordinates<i32> for &BaseInterval {
    fn chr(&self) -> &i32 {
        &0
    }
    fn start(&self) -> i32 {
        self.start
    }
    fn end(&self) -> i32 {
        self.end
    }
    fn update_start(&mut self, _val: &i32) {
        unimplemented!("Cannot update a reference")
    }
    fn update_end(&mut self, _val: &i32) {
        unimplemented!("Cannot update a reference")
    }
    fn update_chr(&mut self, _val: &i32) {
        unimplemented!("Cannot update a reference");
    }
    fn empty() -> Self {
        unimplemented!("Cannot create a reference");
    }
    fn from<Iv: Coordinates<i32>>(_other: &Iv) -> Self {
        unimplemented!("Cannot create a reference")
    }
}
impl Coordinates<i32> for &mut BaseInterval {
    fn chr(&self) -> &i32 {
        &0
    }
    fn start(&self) -> i32 {
        self.start
    }
    fn end(&self) -> i32 {
        self.end
    }
    fn update_start(&mut self, val: &i32) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &i32) {
        self.end = *val;
    }
    fn update_chr(&mut self, _val: &i32) {
        // Do nothing
    }
    fn empty() -> Self {
        unimplemented!("Cannot create a reference");
    }
    fn from<Iv: Coordinates<i32>>(_other: &Iv) -> Self {
        unimplemented!("Cannot create a reference")
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
        assert_eq!(format!("{iv:?}"), "BaseInterval { start: 10, end: 100 }");
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
        let b: BaseInterval = deserialize(&encoding).unwrap();
        assert!(a.eq(&b));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn function_generic_reference<C: Coordinates<i32>>(iv: C) {
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
        let b: BaseInterval = iter.next().unwrap()?;
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        Ok(())
    }
}
