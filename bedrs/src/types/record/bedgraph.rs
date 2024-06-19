use crate::{
    bed3,
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed3, Bed6, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use getset::{CopyGetters, Getters, Setters};
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
#[derive(Debug, Default, Clone, Copy, Coordinates, Getters, Setters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BedGraph<C>
where
    C: ChromBounds,
{
    chr: C,
    start: i32,
    end: i32,
    #[getset(get_copy = "pub", set = "pub")]
    score: f64,
}

impl<C> From<BedGraph<C>> for Bed3<C>
where
    C: ChromBounds,
{
    fn from(bed: BedGraph<C>) -> Self {
        bed3![bed.chr, bed.start, bed.end]
    }
}

impl<C, N> From<BedGraph<C>> for Bed6<C, N>
where
    C: ChromBounds,
    N: MetaBounds + std::convert::From<f64>,
{
    fn from(bed: BedGraph<C>) -> Self {
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

impl<C, N, Ts, Te, R, T, Si, St> From<BedGraph<C>> for Bed12<C, N, Ts, Te, R, T, Si, St>
where
    C: ChromBounds,
    N: MetaBounds + From<f64>,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(bed: BedGraph<C>) -> Self {
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
    use crate::bed4;

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
        let b3: Bed3<_> = b.into();
        assert_eq!(b3.chr(), &1);
        assert_eq!(b3.start(), 10);
        assert_eq!(b3.end(), 20);
    }

    #[test]
    fn convert_to_bed4() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b4: Bed6<_, f64> = b.into();
        assert_eq!(b4.chr(), &1);
        assert_eq!(b4.start(), 10);
        assert_eq!(b4.end(), 20);
        assert!(float_eq(*b4.name(), 66.6));
    }

    #[test]
    fn convert_to_bed6() {
        let b = BedGraph::new(1, 10, 20, 66.6);
        let b6: Bed6<i32, f64> = b.into();
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
        let b12: Bed12<i32, f64, i32, i32, i32, i32, i32, i32> = b.into();
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
        let b3 = bed3![1, 10, 20];
        let bg: BedGraph<_> = b3.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 0.));
    }

    #[test]
    fn from_bed4() {
        let b4 = bed4![1, 10, 20, 66.6];
        let bg: BedGraph<_> = b4.into();
        assert_eq!(bg.chr(), &1);
        assert_eq!(bg.start(), 10);
        assert_eq!(bg.end(), 20);
        assert!(float_eq(bg.score(), 66.6));
    }

    #[test]
    fn from_bed6() {
        let b6 = Bed6::new(1, 10, 20, 66.6, 30.into(), Strand::Unknown);
        let bg: BedGraph<_> = b6.into();
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
        let bg: BedGraph<_> = b12.into();
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
        let b: BedGraph<String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert!(float_eq(b.score(), 66.6));
        Ok(())
    }
}
