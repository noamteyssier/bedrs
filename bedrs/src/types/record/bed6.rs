use crate::{
    bed3, bed4,
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed3, Bed4, BedGraph, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use getset::{CopyGetters, Getters, Setters};
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
/// assert_eq!(*a.metadata.name(), 10);
///
/// let b = Bed4::new(1, 20, 30, 0);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Default, Clone, Copy, Coordinates, new, Getters, Setters, CopyGetters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed6<C, N>
where
    C: ChromBounds,
    N: MetaBounds,
{
    chr: C,
    start: i32,
    end: i32,
    #[getset(get = "pub", set = "pub")]
    name: N,
    #[getset(get_copy = "pub", set = "pub")]
    score: Score,
    strand: Strand,
}

impl<C, N> From<Bed6<C, N>> for Bed3<C>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(bed: Bed6<C, N>) -> Self {
        bed3![bed.chr, bed.start, bed.end]
    }
}

impl<C, N> From<Bed6<C, N>> for Bed4<C, N>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(bed: Bed6<C, N>) -> Self {
        bed4![bed.chr, bed.start, bed.end, bed.name]
    }
}

impl<C, N> From<Bed6<C, N>> for BedGraph<C>
where
    C: ChromBounds,
    N: MetaBounds,
    f64: From<N>,
{
    fn from(bed: Bed6<C, N>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, bed.name.into())
    }
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed6<C, N>> for Bed12<C, N, Ts, Te, R, T, Si, St>
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
    fn from(bed: Bed6<C, N>) -> Self {
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
    use crate::{bed4, bed6, IntervalContainer};

    use super::*;

    #[test]
    fn test_init_chrom_numeric() {
        let a = bed6![1, 10, 20, 0, Score(None), Strand::Unknown];
        assert_eq!(a.chr(), &1);
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.metadata.name(), &0);
        assert_eq!(*a.metadata.score(), Score(None));
        assert_eq!(a.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn test_init_chrom_string() {
        let a = bed6!["chr1".to_string(), 10, 20, 0, Score(None), Strand::Unknown];
        assert_eq!(a.chr(), &"chr1".to_string());
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.metadata.name(), &0);
        assert_eq!(*a.metadata.score(), Score(None));
        assert_eq!(a.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn test_init_name_numeric() {
        let a = bed6![1, 10, 20, 0, Score(None), Strand::Unknown];
        assert_eq!(a.metadata.name(), &0);
    }

    #[test]
    fn test_init_name_string() {
        let a = bed6![1, 10, 20, "name".to_string(), Score(None), Strand::Unknown];
        assert_eq!(a.metadata.name(), &"name".to_string());
    }

    #[test]
    fn test_init_score_discrete() {
        let a = bed6![1, 10, 20, "name".to_string(), 11.into(), Strand::Unknown];
        assert_eq!(*a.metadata.score(), 11.into());
    }

    #[test]
    fn test_init_score_continuous() {
        let a = bed6![1, 10, 20, "name".to_string(), 11.1.into(), Strand::Unknown];
        assert_eq!(*a.metadata.score(), 11.1.into());
    }

    #[test]
    fn convert_to_bed3() {
        let a = bed6![1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward];
        let b: Bed3<i32> = (&a).into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
    }

    #[test]
    fn convert_to_bed4() {
        let a = bed6![1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward];
        let b: Bed4<i32, String> = (&a).into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.metadata.name(), "name");
    }

    #[test]
    fn convert_to_bed12() {
        let a = bed6![1, 10, 20, "name".to_string(), 11.1.into(), Strand::Forward];
        let b: Bed12<i32, String, i32, i32, f32, i32, i32, i32> = a.into();
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
        let a = bed3![1, 10, 20];
        let b: Bed6<i32, String> = a.into();
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "");
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn from_bed4() {
        let a = bed4![1, 10, 20, "name".to_string()];
        let b: Bed6<i32, String> = a.into();
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
        let b: Bed6<i32, String> = a.into();
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
            bed6![1, 10, 20, "name".to_string(), 0.into(), Strand::Forward],
            bed6![1, 15, 25, "name".to_string(), 0.into(), Strand::Forward],
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
        let a = bed6![1, 20, 30, "metadata", 0.into(), Strand::Forward];
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
        let b: Bed6<i32, String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), "metadata");
        assert_eq!(b.score(), 0.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
        Ok(())
    }
}
