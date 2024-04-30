use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed3, Bed6, BedGraph, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use getset::{Getters, Setters};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Bed4 Interval.
///
/// Has four values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `name`
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
#[derive(Debug, Default, Clone, Copy, Coordinates, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    chr: C,
    start: T,
    end: T,
    #[getset(get = "pub", set = "pub")]
    name: N,
}

impl<C, T, N> From<Bed4<C, T, N>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed4<C, T, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end)
    }
}

impl<C, T, N> From<Bed4<C, T, N>> for BedGraph<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    f64: From<N>,
{
    fn from(bed: Bed4<C, T, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, bed.name.into())
    }
}

impl<C, T, N> From<Bed4<C, T, N>> for Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed4<C, T, N>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            bed.name,
            Score::default(),
            Strand::Unknown,
        )
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed4<C, T, N>> for Bed12<C, T, N, Ts, Te, R, Si, St>
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
    fn from(bed: Bed4<C, T, N>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            bed.name,
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

    #[test]
    fn test_init_chrom_numeric() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        assert_eq!(b.chr(), &1);
    }

    #[test]
    fn test_init_chrom_string() {
        let b = Bed4::new("chr1".to_string(), 10, 20, "test".to_string());
        assert_eq!(b.chr(), "chr1");
    }

    #[test]
    fn test_init_name_numeric() {
        let b = Bed4::new(1, 10, 20, 30);
        assert_eq!(b.name(), &30);
    }

    #[test]
    fn test_init_name_string() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        assert_eq!(b.name(), "test");
    }

    #[test]
    fn convert_to_bed3() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b3: Bed3<_, _> = b.into();
        assert_eq!(b3.chr(), &1);
        assert_eq!(b3.start(), 10);
        assert_eq!(b3.end(), 20);
    }

    #[test]
    fn convert_to_bed6() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b6: Bed6<i32, i32, String> = b.into();
        assert_eq!(b6.chr(), &1);
        assert_eq!(b6.start(), 10);
        assert_eq!(b6.end(), 20);
        assert_eq!(b6.name(), "test");
        assert_eq!(b6.score(), Score(None));
        assert_eq!(b6.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn convert_to_bed12() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b12: Bed12<i32, i32, String, i32, i32, i32, i32, i32> = b.into();
        assert_eq!(b12.chr(), &1);
        assert_eq!(b12.start(), 10);
        assert_eq!(b12.end(), 20);
        assert_eq!(b12.name(), "test");
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
        let b4: Bed4<_, _, String> = b3.into();
        assert_eq!(b4.chr(), &1);
        assert_eq!(b4.start(), 10);
        assert_eq!(b4.end(), 20);
        assert_eq!(b4.name(), "");
    }

    #[test]
    fn from_bed6() {
        let b6 = Bed6::new(1, 10, 20, "test".to_string(), 30.into(), Strand::Unknown);
        let b4: Bed4<_, _, String> = b6.into();
        assert_eq!(b4.chr(), &1);
        assert_eq!(b4.start(), 10);
        assert_eq!(b4.end(), 20);
        assert_eq!(b4.name(), "test");
    }

    #[test]
    fn from_bed12() {
        let b12 = Bed12::new(
            1,
            10,
            20,
            "test".to_string(),
            30.into(),
            Strand::Unknown,
            40,
            50,
            60,
            70,
            80,
            90,
        );
        let b4: Bed4<_, _, String> = b12.into();
        assert_eq!(b4.chr(), &1);
        assert_eq!(b4.start(), 10);
        assert_eq!(b4.end(), 20);
        assert_eq!(b4.name(), "test");
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
        let a = Bed4::new("chr1", 20, 30, "metadata");
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
        let b: Bed4<String, i32, String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), "metadata");
        Ok(())
    }
}
