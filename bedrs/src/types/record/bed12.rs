use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed3, Bed4, Bed6, BedGraph, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use getset::{CopyGetters, Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A representation of a Bed12 interval.
///
/// Has twelve values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `name`
///     5. `score`
///     6. `strand`
///     7. `thick_start`
///     8. `thick_end`
///     9. `item_rgb`
///     10. `block_count`
///     11. `block_sizes`
///     12. `block_starts`
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
#[allow(clippy::too_many_arguments)]
#[derive(Debug, Default, Clone, Copy, Coordinates, Getters, Setters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed12<C, N, Ts, Te, R, T, Si, St>
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
    chr: C,
    start: i32,
    end: i32,
    #[getset(get = "pub", set = "pub")]
    name: N,
    #[getset(get_copy = "pub", set = "pub")]
    score: Score,
    strand: Strand,
    #[getset(get_copy = "pub", set = "pub")]
    thick_start: Ts,
    #[getset(get_copy = "pub", set = "pub")]
    thick_end: Te,
    #[getset(get = "pub", set = "pub")]
    item_rgb: R,
    #[getset(get_copy = "pub", set = "pub")]
    block_count: T,
    #[getset(get = "pub", set = "pub")]
    block_sizes: Si,
    #[getset(get = "pub", set = "pub")]
    block_starts: St,
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed12<C, N, Ts, Te, R, T, Si, St>> for Bed3<C>
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
    fn from(bed: Bed12<C, N, Ts, Te, R, T, Si, St>) -> Self {
        Bed3::new(bed.chr, bed.start, bed.end)
    }
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed12<C, N, Ts, Te, R, T, Si, St>> for Bed4<C, N>
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
    fn from(bed: Bed12<C, N, Ts, Te, R, T, Si, St>) -> Self {
        Bed4::new(bed.chr, bed.start, bed.end, bed.name)
    }
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed12<C, N, Ts, Te, R, T, Si, St>> for BedGraph<C>
where
    C: ChromBounds,
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
    f64: From<N>,
{
    fn from(bed: Bed12<C, N, Ts, Te, R, T, Si, St>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, bed.name.into())
    }
}

impl<C, N, Ts, Te, R, T, Si, St> From<Bed12<C, N, Ts, Te, R, T, Si, St>> for Bed6<C, N>
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
    fn from(bed: Bed12<C, N, Ts, Te, R, T, Si, St>) -> Self {
        Bed6::new(bed.chr, bed.start, bed.end, bed.name, bed.score, bed.strand)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod testing {
    use super::*;

    #[test]
    fn test_init() {
        let a: Bed12<String, usize, usize, usize, usize, usize, usize, usize> = Bed12::empty();
        let b = a.clone();
        assert_eq!(a.chr(), b.chr());
        assert_eq!(a.start(), b.start());
        assert_eq!(a.end(), b.end());
        assert_eq!(a.name(), b.name());
        assert_eq!(a.score(), b.score());
        assert_eq!(a.strand().unwrap(), b.strand().unwrap());
        assert_eq!(a.thick_start(), b.thick_start());
        assert_eq!(a.thick_end(), b.thick_end());
        assert_eq!(a.item_rgb(), b.item_rgb());
        assert_eq!(a.block_count(), b.block_count());
        assert_eq!(a.block_sizes(), b.block_sizes());
        assert_eq!(a.block_starts(), b.block_starts());
        assert_eq!(
            format!("{a:?}"),
            "Bed12 { chr: \"\", start: 0, end: 0, name: 0, score: Score(None), strand: Unknown, thick_start: 0, thick_end: 0, item_rgb: 0, block_count: 0, block_sizes: 0, block_starts: 0 }"
        );
    }

    #[test]
    fn test_updates() {
        let mut a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        a.update_chr(&String::from("chr2"));
        a.update_start(&20);
        a.update_end(&30);
        a.update_strand(Some(Strand::Reverse));
        a.set_name(String::from("new_name"));
        a.set_score(2.into());
        a.set_thick_start(2);
        a.set_thick_end(3);
        a.set_item_rgb(String::from("1,1,1"));
        a.set_block_count(2);
        a.set_block_sizes(vec![1, 2]);
        a.set_block_starts(vec![1, 2]);
        assert_eq!(a.chr(), "chr2");
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
        assert_eq!(a.name(), "new_name");
        assert_eq!(a.score(), 2.into());
        assert_eq!(a.strand().unwrap(), Strand::Reverse);
        assert_eq!(a.thick_start(), 2);
        assert_eq!(a.thick_end(), 3);
        assert_eq!(a.item_rgb(), "1,1,1");
        assert_eq!(a.block_count(), 2);
        assert_eq!(a.block_sizes(), &vec![1, 2]);
        assert_eq!(a.block_starts(), &vec![1, 2]);
    }

    #[test]
    fn test_init_chrom_string() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), "chr1");
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), "name");
        assert_eq!(a.score(), 1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn test_init_chrom_numeric() {
        let a = Bed12::new(
            1,
            10,
            20,
            "name".to_string(),
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), &1);
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), "name");
        assert_eq!(a.score(), 1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn test_init_name_string() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), "chr1");
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), "name");
        assert_eq!(a.score(), 1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn test_init_name_numeric() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            1,
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), "chr1");
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), &1);
        assert_eq!(a.score(), 1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn test_init_score_discrete() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), "chr1");
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), "name");
        assert_eq!(a.score(), 1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn test_init_score_continuous() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        assert_eq!(a.chr(), "chr1");
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 20);
        assert_eq!(a.name(), "name");
        assert_eq!(a.score(), 1.1.into());
        assert_eq!(a.strand().unwrap(), Strand::Forward);
        assert_eq!(a.thick_start(), 1);
        assert_eq!(a.thick_end(), 2);
        assert_eq!(a.item_rgb(), "0,0,0");
        assert_eq!(a.block_count(), 1);
        assert_eq!(a.block_sizes(), &vec![1]);
        assert_eq!(a.block_starts(), &vec![1]);
    }

    #[test]
    fn convert_to_bed3() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        let b: Bed3<String> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
    }

    #[test]
    fn convert_to_bed4() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        let b: Bed4<String, String> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
    }

    #[test]
    fn convert_to_bed6() {
        let a = Bed12::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.1.into(),
            Strand::Forward,
            1,
            2,
            "0,0,0".to_string(),
            1,
            vec![1],
            vec![1],
        );
        let b: Bed6<String, String> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), 1.1.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn from_bed3() {
        let a = Bed3::new("chr1".to_string(), 10, 20);
        let b: Bed12<String, String, i32, i32, String, i32, Vec<i32>, Vec<i32>> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "");
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), "");
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &Vec::<i32>::new());
        assert_eq!(b.block_starts(), &Vec::<i32>::new());
    }

    #[test]
    fn from_bed4() {
        let a = Bed4::new("chr1".to_string(), 10, 20, "name".to_string());
        let b: Bed12<String, String, i32, i32, String, i32, Vec<i32>, Vec<i32>> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), "");
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &Vec::<i32>::new());
        assert_eq!(b.block_starts(), &Vec::<i32>::new());
    }

    #[test]
    fn from_bed6() {
        let a = Bed6::new(
            "chr1".to_string(),
            10,
            20,
            "name".to_string(),
            1.1.into(),
            Strand::Forward,
        );
        let b: Bed12<String, String, i32, i32, String, i32, Vec<i32>, Vec<i32>> = a.into();
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 10);
        assert_eq!(b.end(), 20);
        assert_eq!(b.name(), "name");
        assert_eq!(b.score(), 1.1.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), "");
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &Vec::<i32>::new());
        assert_eq!(b.block_starts(), &Vec::<i32>::new());
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
        let a = Bed12::new(
            "chr1".to_string(),
            20,
            30,
            "metadata".to_string(),
            1.1.into(),
            Strand::Forward,
            20,
            30,
            "0,0,0".to_string(),
            1,
            vec![10],
            vec![20],
        );
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(
            result,
            "chr1,20,30,metadata,1.1,+,20,30,\"0,0,0\",1,10,20\n"
        );
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "chr1\t20\t30\tmetadata\t1.1\t+\t20\t30\tcolor\t1\t2,2\t2,2\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: Bed12<String, String, i32, i32, String, i32, String, String> =
            iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), "metadata");
        assert_eq!(b.score(), 1.1.into());
        assert_eq!(b.strand().unwrap(), Strand::Forward);
        assert_eq!(b.thick_start(), 20);
        assert_eq!(b.thick_end(), 30);
        assert_eq!(b.item_rgb(), "color");
        assert_eq!(b.block_count(), 1);
        assert_eq!(b.block_sizes(), &"2,2");
        assert_eq!(b.block_starts(), &"2,2");
        Ok(())
    }
}
