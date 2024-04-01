use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    Bed3, Coordinates, Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Meta Interval.
///
/// I.e. an interval that has some associated meta value
/// and has three coordinates: `chr`, `start`, and `end`.
///
/// The meta value can be most anything but is bounded by `MetaBounds`.
///
/// # Usage
/// ```
/// use bedrs::{Coordinates, MetaInterval, Overlap};
///
/// let a = MetaInterval::new(1, 20, 30, ("test", 0, '.'));
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(a.meta(), &("test", 0, '.'));
///
/// let b = MetaInterval::new(1, 20, 30, ("something_else", 20, '.'));
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaInterval<C, T, M> {
    chr: C,
    start: T,
    end: T,
    meta: M,
}
impl<C, T, M> Coordinates<C, T> for MetaInterval<C, T, M>
where
    C: ChromBounds,
    T: ValueBounds,
    M: MetaBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            meta: M::default(),
        }
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &C) {
        self.chr = val.clone();
    }
    fn from<Iv: Coordinates<C, T>>(other: &Iv) -> Self {
        Self {
            chr: other.chr().clone(),
            start: other.start(),
            end: other.end(),
            meta: M::default(),
        }
    }
}
impl<'a, C, T, M> Coordinates<C, T> for &'a MetaInterval<C, T, M>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable empty reference")
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
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
    fn update_chr(&mut self, val: &C) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn update_strand(&mut self, strand: Option<Strand>) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}
impl<'a, C, T, M> Coordinates<C, T> for &'a mut MetaInterval<C, T, M>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable empty reference")
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &C) {
        self.chr = val.clone();
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
    }
}

impl<C, T, M> MetaInterval<C, T, M>
where
    C: ChromBounds,
    T: ValueBounds,
    M: MetaBounds,
{
    pub fn new(chr: C, start: T, end: T, meta: M) -> Self {
        Self {
            chr,
            start,
            end,
            meta,
        }
    }
    pub fn meta(&self) -> &M {
        &self.meta
    }
    pub fn update_meta(&mut self, val: &M) {
        self.meta = val.clone();
    }
}

impl<C, T, M> From<MetaInterval<C, T, M>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    M: MetaBounds,
{
    fn from(bed: MetaInterval<C, T, M>) -> Self {
        Self::new(bed.chr, bed.start, bed.end)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_init_numeric() {
        let a = MetaInterval::new(1, 20, 30, 100);
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
        assert_eq!(a.chr(), &1);
        assert_eq!(a.meta(), &100);
    }

    #[test]
    fn test_init_tuple_meta() {
        let a = MetaInterval::new(1, 20, 30, (100, 200, "test"));
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
        assert_eq!(a.chr(), &1);
        assert_eq!(a.meta(), &(100, 200, "test"));
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
        let a = MetaInterval::new("chr1", 20, 30, "metadata");
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "chr1,20,30,metadata\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "chr1,20,30,metadata\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: MetaInterval<String, i32, String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.meta(), "metadata");
        Ok(())
    }
}
