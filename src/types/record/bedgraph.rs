use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed3, Bed6, Coordinates, Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a `BedGraph` Interval.
///
/// Has four values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `score`
///
/// ```
/// use bedrs::{Coordinates, BedGraph, Overlap};
///
/// let a = BedGraph::new(1, 20, 30, 10.0);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(a.score(), 10.0);
///
/// let b = BedGraph::new(1, 20, 30, 0.0);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BedGraph<C, T> {
    chr: C,
    start: T,
    end: T,
    score: f64,
}

impl<C, T> Coordinates<C, T> for BedGraph<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            score: f64::default(),
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
            score: f64::default(),
        }
    }
}
impl<'a, C, T> Coordinates<C, T> for &'a BedGraph<C, T>
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
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}
impl<'a, C, T> Coordinates<C, T> for &'a mut BedGraph<C, T>
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

impl<C, T> BedGraph<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(chr: C, start: T, end: T, score: f64) -> Self {
        Self {
            chr,
            start,
            end,
            score,
        }
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn update_score(&mut self, val: f64) {
        self.score = val;
    }
}

impl<C, T> From<BedGraph<C, T>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn from(bed: BedGraph<C, T>) -> Self {
        Self::new(bed.chr, bed.start, bed.end)
    }
}

impl<C, T, N> From<BedGraph<C, T>> for Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds + std::convert::From<f64>,
{
    fn from(bed: BedGraph<C, T>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            bed.score.into(),
            Score::default(),
            Strand::Unknown,
        )
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<BedGraph<C, T>> for Bed12<C, T, N, Ts, Te, R, Si, St>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds + From<f64>,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(bed: BedGraph<C, T>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            bed.score.into(),
            Score::default(),
            Strand::Unknown,
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
mod testing {
    use super::*;
    use crate::Bed4;

    fn float_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_init_chrom_numeric() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        assert_eq!(b.chr(), &1);
    }

    #[test]
    fn test_init_chrom_string() {
        let b = BedGraph::new("chr1".to_string(), 10, 20, 66.6);
        assert_eq!(b.chr(), "chr1");
    }

    #[test]
    fn test_init_score_numeric() {
        let b = BedGraph::new(1, 10, 20, 30.0);
        assert!(float_eq(b.score(), 30.));
    }

    #[test]
    fn test_init_score_string() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        assert!(float_eq(b.score(), 66.6));
    }

    #[test]
    fn convert_to_bed3() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b3: Bed3<_, _> = b.into();
        assert_eq!(b3.chr(), &1);
        assert_eq!(b3.start(), 10);
        assert_eq!(b3.end(), 20);
    }

    #[test]
    fn convert_to_bed4() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b4: Bed6<_, _, f64> = b.into();
        assert_eq!(b4.chr(), &1);
        assert_eq!(b4.start(), 10);
        assert_eq!(b4.end(), 20);
        assert!(float_eq(*b4.name(), 66.6));
    }

    #[test]
    fn convert_to_bed6() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b6: Bed6<i32, i32, f64> = b.into();
        assert_eq!(b6.chr(), &1);
        assert_eq!(b6.start(), 10);
        assert_eq!(b6.end(), 20);
        assert_eq!(b6.score(), Score(None));
        assert_eq!(b6.strand().unwrap(), Strand::Unknown);
        assert!(float_eq(*b6.name(), 66.6));
    }

    #[test]
    fn convert_to_bed12() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b12: Bed12<i32, i32, f64, i32, i32, i32, i32, i32> = b.into();
        assert_eq!(b12.chr(), &1);
        assert_eq!(b12.start(), 10);
        assert_eq!(b12.end(), 20);
        assert!(float_eq(*b12.name(), 66.6));
        assert_eq!(b12.score(), Score(None));
        assert_eq!(b12.strand().unwrap(), Strand::Unknown);
        assert_eq!(b12.thick_start(), 0);
        assert_eq!(b12.thick_end(), 0);
        assert_eq!(b12.item_rgb(), &0);
        assert_eq!(b12.block_count(), 0);
        assert_eq!(b12.block_sizes(), &0);
        assert_eq!(b12.block_starts(), &0);
    }

    #[test]
    fn from_bed3() {
        let b3 = Bed3::new(1, 10, 20);
        let bg: BedGraph<_, _> = b3.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 0.));
    }

    #[test]
    fn from_bed4() {
        let b4 = Bed4::new(1, 10, 20, 66.6);
        let bg: BedGraph<_, _> = b4.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 66.6));
    }

    #[test]
    fn from_bed6() {
        let b6 = Bed6::new(1, 10, 20, 66.6, 30.into(), Strand::Unknown);
        let bg: BedGraph<_, _> = b6.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 66.6));
    }

    #[test]
    fn from_bed12() {
        let b12 = Bed12::new(
            1,
            10,
            20,
            66.6,
            30.into(),
            Strand::Unknown,
            40,
            50,
            60,
            70,
            80,
            90,
        );
        let bg: BedGraph<_, _> = b12.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 66.6));
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_testing {
    use super::*;
    use anyhow::Result;
    use csv::WriterBuilder;

    fn float_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_csv_serialization() -> Result<()> {
        let a = BedGraph::new("chr1", 20, 30, 66.6);
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "chr1,20,30,66.6\n");
        Ok(())
    }

    #[test]
    #[allow(unknown_lints)]
    #[allow(clippy::float_cmp)]
    fn test_csv_deserialization() -> Result<()> {
        let a = "chr1,20,30,66.6\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: BedGraph<String, i32> = iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert!(float_eq(b.score(), 66.6));
        Ok(())
    }
}
