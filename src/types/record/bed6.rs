use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed3, Bed4, BedGraph, Coordinates, Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Bed4 Interval.
///
/// Has six values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `name`
///     5. `score`
///     6. `strand`
///
/// ```
/// use bedrs::{Coordinates, Bed4, Overlap};
///
/// let a = Bed4::new(1, 20, 30, 10);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(*a.name(), 10);
///
/// let b = Bed4::new(1, 20, 30, 0);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed6<C, T, N> {
    chr: C,
    start: T,
    end: T,
    name: N,
    score: Score,
    strand: Strand,
}

impl<C, T, N> Coordinates<C, T> for Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            name: N::default(),
            score: Score::default(),
            strand: Strand::Unknown,
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
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
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
    fn update_strand(&mut self, strand: Option<Strand>) {
        self.strand = strand.unwrap_or_default();
    }
    fn from<Iv: Coordinates<C, T>>(other: &Iv) -> Self {
        Self {
            chr: other.chr().clone(),
            start: other.start(),
            end: other.end(),
            name: N::default(),
            score: Score::default(),
            strand: other.strand().unwrap_or_default(),
        }
    }
}
impl<'a, C, T, N> Coordinates<C, T> for &'a Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
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
impl<'a, C, T, N> Coordinates<C, T> for &'a mut Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
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
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
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
    fn update_strand(&mut self, strand: Option<Strand>) {
        self.strand = strand.unwrap_or_default();
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
    }
}

impl<C, T, N> Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    pub fn new(chr: C, start: T, end: T, name: N, score: Score, strand: Strand) -> Self {
        Self {
            chr,
            start,
            end,
            name,
            score,
            strand,
        }
    }

    pub fn name(&self) -> &N {
        &self.name
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn update_name(&mut self, val: &N) {
        self.name = val.clone();
    }

    pub fn update_score(&mut self, val: Score) {
        self.score = val;
    }
}

impl<C, T, N> From<Bed6<C, T, N>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed6<C, T, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end)
    }
}

impl<C, T, N> From<Bed6<C, T, N>> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed6<C, T, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, bed.name)
    }
}

impl<C, T, N> From<Bed6<C, T, N>> for BedGraph<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    f64: From<N>,
{
    fn from(bed: Bed6<C, T, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, bed.name.into())
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed6<C, T, N>> for Bed12<C, T, N, Ts, Te, R, Si, St>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(bed: Bed6<C, T, N>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            bed.name,
            bed.score,
            bed.strand,
            Ts::default(),
            Te::default(),
            R::default(),
            zero::<T>(),
            Si::default(),
            St::default(),
        )
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod testing {
    use crate::IntervalContainer;

    use super::*;

    #[test]
    fn test_init_chrom_numeric() {
        let a = Bed6::new(1, 10, 20, 0, Score(None), Strand::Unknown);
        assert_eq!(a.chr(), &1);
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), &0);
        assert_eq!(a.score(), Score(None));
        assert_eq!(a.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn test_init_chrom_string() {
        let a = Bed6::new("chr1".to_string(), 10, 20, 0, Score(None), Strand::Unknown);
        assert_eq!(a.chr(), &"chr1".to_string());
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), &0);
        assert_eq!(a.score(), Score(None));
        assert_eq!(a.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn test_init_name_numeric() {
        let a = Bed6::new(1, 10, 20, 0, Score(None), Strand::Unknown);
        assert_eq!(a.name(), &0);
    }

    #[test]
    fn test_init_name_string() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), Score(None), Strand::Unknown);
        assert_eq!(a.name(), &"name".to_string());
    }

    #[test]
    fn test_init_score_discrete() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), 11.into(), Strand::Unknown);
        assert_eq!(a.score(), 11.into());
    }

    #[test]
    fn test_init_score_continuous() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), 11.1.into(), Strand::Unknown);
        assert_eq!(a.score(), 11.1.into());
    }

    #[test]
    fn convert_to_bed3() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward);
        let b: Bed3<i32, i32> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
    }

    #[test]
    fn convert_to_bed4() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward);
        let b: Bed4<i32, i32, String> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
    }

    #[test]
    fn convert_to_bed12() {
        let a = Bed6::new(1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward);
        let b: Bed12<i32, i32, String, i32, i32, f32, i32, i32> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), 11.1.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), &0.0);
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &0);
        assert_eq!(b.block_starts(), &0);
    }

    #[test]
    fn from_bed3() {
        let a = Bed3::new(1, 10, 20);
        let b: Bed6<i32, i32, String> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "");
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn from_bed4() {
        let a = Bed4::new(1, 10, 20, "name".to_string());
        let b: Bed6<i32, i32, String> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn from_bed12() {
        let a = Bed12::new(
            1,
            10,
            20,
            "name".to_string(),
            11.1.into(),
            Strand::Forward,
            0,
            0,
            0,
            0,
            0,
            0,
        );
        let b: Bed6<i32, i32, String> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), 11.1.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn merge_bed6() {
        let set = IntervalContainer::from_sorted_unchecked(vec![
            Bed6::new(1, 10, 20, "name".to_string(), 0.into(), Strand::Forward),
            Bed6::new(1, 15, 25, "name".to_string(), 0.into(), Strand::Forward),
        ]);
        let merged = set.merge_unchecked();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged.records()[0].start(), 10);
        assert_eq!(merged.records()[0].end(), 25);
        assert_eq!(merged.records()[0].name(), "");
        assert_eq!(merged.records()[0].strand().unwrap(), Strand::Forward);
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
        let a = Bed6::new(1, 20, 30, "metadata", 0.into(), Strand::Forward);
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "1,20,30,metadata,0,+\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "1,20,30,metadata,0,+\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: Bed6<i32, i32, String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), "metadata");
        assert_eq!(b.score(), 0.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
        Ok(())
    }
}
