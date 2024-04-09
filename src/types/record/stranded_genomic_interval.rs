use crate::{
    traits::{Coordinates, ValueBounds},
    Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Genomic Interval.
///
/// Has three coordinates: `chr`, `start`, and `end`.
/// Has an associated `Strand` which can be either `Forward` or `Reverse`.
/// This is an associated metadata and is not used in comparisons.
///
/// ```
/// use bedrs::{Coordinates, StrandedBed3, Overlap, Strand, StrandedOverlap};
///
/// let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(a.strand(), Some(Strand::Forward));
///
/// let b = StrandedBed3::new(1, 20, 30, Strand::Reverse);
/// assert!(a.overlaps(&b));
/// assert!(!a.stranded_overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrandedBed3<T> {
    chr: T,
    start: T,
    end: T,
    strand: Strand,
}

impl<T> Coordinates<T, T> for StrandedBed3<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        Self {
            chr: zero::<T>(),
            start: zero::<T>(),
            end: zero::<T>(),
            strand: Strand::Forward,
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
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
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
    fn update_strand(&mut self, strand: Option<Strand>) {
        self.strand = strand.unwrap_or_default();
    }
    fn from<Iv: Coordinates<T, T>>(other: &Iv) -> Self {
        Self {
            chr: *other.chr(),
            start: other.start(),
            end: other.end(),
            strand: other.strand().unwrap_or_default(),
        }
    }
}

impl<'a, T> Coordinates<T, T> for &'a StrandedBed3<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an empty reference")
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
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
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

impl<'a, T> Coordinates<T, T> for &'a mut StrandedBed3<T>
where
    T: ValueBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an empty reference")
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
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
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
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}

impl<T> StrandedBed3<T>
where
    T: ValueBounds,
{
    /// Create a new `StrandedBed3`
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, StrandedBed3, Strand};
    /// let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
    /// assert_eq!(*a.chr(), 1);
    /// assert_eq!(a.start(), 20);
    /// assert_eq!(a.end(), 30);
    /// assert_eq!(a.strand(), Some(Strand::Forward));
    /// ```
    pub fn new(chr: T, start: T, end: T, strand: Strand) -> Self {
        Self {
            chr,
            start,
            end,
            strand,
        }
    }

    /// Overwrite the strand of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, StrandedBed3, Strand};
    ///
    /// let mut a = StrandedBed3::new(1, 20, 30, Strand::Forward);
    /// assert_eq!(a.strand(), Some(Strand::Forward));
    /// a.set_strand(Strand::Reverse);
    /// assert_eq!(a.strand(), Some(Strand::Reverse));
    /// ```
    pub fn set_strand(&mut self, strand: Strand) {
        self.strand = strand;
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::Coordinates,
        types::{Strand, StrandedBed3},
        Subtract,
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 90, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedBed3::new(2, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(2, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = StrandedBed3::new(2, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(2, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    fn test_set_strand() {
        let mut a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.strand(), Some(Strand::Forward));
        a.set_strand(Strand::Reverse);
        assert_eq!(a.strand(), Some(Strand::Reverse));
    }

    #[test]
    fn test_subtraction_a() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_b() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_c() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b = StrandedBed3::new(1, 10, 100, Strand::Reverse);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Reverse);
    }

    #[test]
    fn test_from() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b: StrandedBed3<usize> = Coordinates::from(&a);
        assert!(a.eq(&b));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn stranded_genomic_interval_serde() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let serialized = serialize(&a).unwrap();
        let deserialized: StrandedBed3<u32> = deserialize(&serialized).unwrap();
        assert!(a.eq(&deserialized));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_some());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = StrandedBed3::new(1, 10, 100, Strand::Forward);
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
        let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "1,20,30,+\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "1,20,30,+\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: StrandedBed3<i32> = iter.next().unwrap()?;
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.strand(), Some(Strand::Forward));
        Ok(())
    }
}
