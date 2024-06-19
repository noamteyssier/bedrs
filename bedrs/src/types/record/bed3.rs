use crate::{
    bed4,
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed4, Bed6, BedGraph, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Bed3 Interval.
///
/// Has three values
///     1. `chr`
///     2. `start`
///     3. `end`
///
/// ```
/// use bedrs::{Coordinates, Bed3, Overlap};
///
/// let a = bed3![1, 20, 30];
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = bed3![1, 20, 30];
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Default, Clone, Copy, Coordinates, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed3<C>
where
    C: ChromBounds,
{
    chr: C,
    start: i32,
    end: i32,
}

impl<C, N> From<Bed3<C>> for Bed4<C, N>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(bed: Bed3<C>) -> Self {
        bed4![bed.chr, bed.start, bed.end, N::default()]
    }
}

impl<C> From<Bed3<C>> for BedGraph<C>
where
    C: ChromBounds,
{
    fn from(bed: Bed3<C>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, f64::default())
    }
}

impl<C, N> From<Bed3<C>> for Bed6<C, N>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(bed: Bed3<C>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            N::default(),
            Score::default(),
            Strand::Unknown,
        )
    }
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed3<C>> for Bed12<C, N, Ts, Te, R, T, Si, St>
where
    C: ChromBounds,
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(bed: Bed3<C>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            N::default(),
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
    use crate::bed3;
    use crate::bed4;
    use crate::bed6;
    use crate::types::Bed3;
    use crate::types::Bed4;
    use crate::types::Bed6;
    use crate::Coordinates;
    use crate::Score;
    use crate::Strand;

    #[test]
    fn test_init_numeric() {
        let a = bed3![1, 20, 30];
        assert_eq!(*a.chr(), 1);
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
    }

    #[test]
    fn test_init_named() {
        let a = bed3!["chr1", 20, 30];
        assert_eq!(*a.chr(), "chr1");
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
    }

    #[test]
    fn convert_to_bed4() {
        let a = bed3!["chr1", 20, 30];
        let b: Bed4<_, i32> = (&a).into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.metadata.name(), &0);
    }

    #[test]
    fn convert_to_bed6() {
        let a = bed3!["chr1", 20, 30];
        let b: Bed6<_, i32> = (&a).into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.metadata.name(), &0);
        assert_eq!(*b.metadata.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn convert_to_bed12() {
        let a = bed3!["chr1", 20, 30];
        let b: Bed12<_, i32, i32, i32, i32, i32, i32, i32> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), &0);
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), &0);
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &0);
        assert_eq!(b.block_starts(), &0);
    }

    #[test]
    fn from_bed4() {
        let a = bed4!["chr1", 20, 30, 40i32];
        let b: Bed3<_> = (&a).into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }

    #[test]
    fn from_bed6() {
        let a = bed6!["chr1", 20, 30, 40, 50.into(), Strand::Forward];
        let b: Bed3<_> = (&a).into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }

    #[test]
    fn from_bed12() {
        let a = Bed12::new(
            "chr1",
            20,
            30,
            40,
            50.into(),
            Strand::Forward,
            60,
            70,
            80,
            90,
            100,
            110,
        );
        let b: Bed3<_> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_testing {
    use super::*;
    use crate::bed3;
    use anyhow::Result;
    use csv::WriterBuilder;

    #[test]
    fn test_csv_serialization() -> Result<()> {
        let a = bed3!["chr1", 20, 30];
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "chr1,20,30\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "chr1,20,30\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let r1: Bed3<String> = iter.next().unwrap()?;
        assert_eq!(*r1.chr(), "chr1");
        assert_eq!(r1.start(), 20);
        assert_eq!(r1.end(), 30);
        Ok(())
    }
}
