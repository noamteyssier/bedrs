use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed3, Bed4, Bed6, Coordinates, Strand,
};
use num_traits::zero;
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
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed12<C, T, N, Ts, Te, R, Si, St> {
    chr: C,
    start: T,
    end: T,
    name: N,
    score: Score,
    strand: Strand,
    thick_start: Ts,
    thick_end: Te,
    item_rgb: R,
    block_count: T,
    block_sizes: Si,
    block_starts: St,
}

impl<C, T, N, Ts, Te, R, Si, St> Coordinates<C, T> for Bed12<C, T, N, Ts, Te, R, Si, St>
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
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            name: N::default(),
            score: Score::default(),
            strand: Strand::default(),
            thick_start: zero::<Ts>(),
            thick_end: zero::<Te>(),
            item_rgb: R::default(),
            block_count: zero::<T>(),
            block_sizes: Si::default(),
            block_starts: St::default(),
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
            thick_start: zero::<Ts>(),
            thick_end: zero::<Te>(),
            item_rgb: R::default(),
            block_count: zero::<T>(),
            block_sizes: Si::default(),
            block_starts: St::default(),
        }
    }
}
impl<'a, C, T, N, Ts, Te, R, Si, St> Coordinates<C, T> for &'a Bed12<C, T, N, Ts, Te, R, Si, St>
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
impl<'a, C, T, N, Ts, Te, R, Si, St> Coordinates<C, T> for &'a mut Bed12<C, T, N, Ts, Te, R, Si, St>
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

impl<C, T, N, Ts, Te, R, Si, St> Bed12<C, T, N, Ts, Te, R, Si, St>
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chr: C,
        start: T,
        end: T,
        name: N,
        score: Score,
        strand: Strand,
        thick_start: Ts,
        thick_end: Te,
        item_rgb: R,
        block_count: T,
        block_sizes: Si,
        block_starts: St,
    ) -> Self {
        Self {
            chr,
            start,
            end,
            name,
            score,
            strand,
            thick_start,
            thick_end,
            item_rgb,
            block_count,
            block_sizes,
            block_starts,
        }
    }

    pub fn name(&self) -> &N {
        &self.name
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn thick_start(&self) -> Ts {
        self.thick_start
    }

    pub fn thick_end(&self) -> Te {
        self.thick_end
    }

    pub fn item_rgb(&self) -> &R {
        &self.item_rgb
    }

    pub fn block_count(&self) -> T {
        self.block_count
    }

    pub fn block_sizes(&self) -> &Si {
        &self.block_sizes
    }

    pub fn block_starts(&self) -> &St {
        &self.block_starts
    }

    pub fn update_name(&mut self, val: &N) {
        self.name = val.clone();
    }

    pub fn update_score(&mut self, val: Score) {
        self.score = val;
    }

    pub fn update_thick_start(&mut self, val: &Ts) {
        self.thick_start = *val;
    }

    pub fn update_thick_end(&mut self, val: &Te) {
        self.thick_end = *val;
    }

    pub fn update_item_rgb(&mut self, val: &R) {
        self.item_rgb = val.clone();
    }

    pub fn update_block_count(&mut self, val: &T) {
        self.block_count = *val;
    }

    pub fn update_block_sizes(&mut self, val: &Si) {
        self.block_sizes = val.clone();
    }

    pub fn update_block_starts(&mut self, val: &St) {
        self.block_starts = val.clone();
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed12<C, T, N, Ts, Te, R, Si, St>> for Bed3<C, T>
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
    fn from(bed: Bed12<C, T, N, Ts, Te, R, Si, St>) -> Self {
        Bed3::new(bed.chr, bed.start, bed.end)
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed12<C, T, N, Ts, Te, R, Si, St>> for Bed4<C, T, N>
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
    fn from(bed: Bed12<C, T, N, Ts, Te, R, Si, St>) -> Self {
        Bed4::new(bed.chr, bed.start, bed.end, bed.name)
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed12<C, T, N, Ts, Te, R, Si, St>> for Bed6<C, T, N>
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
    fn from(bed: Bed12<C, T, N, Ts, Te, R, Si, St>) -> Self {
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
        a.update_name(&String::from("new_name"));
        a.update_score(2.into());
        a.update_thick_start(&2);
        a.update_thick_end(&3);
        a.update_item_rgb(&String::from("1,1,1"));
        a.update_block_count(&2);
        a.update_block_sizes(&vec![1, 2]);
        a.update_block_starts(&vec![1, 2]);
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
        let b: Bed3<String, i32> = a.into();
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
        let b: Bed4<String, i32, String> = a.into();
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
        let b: Bed6<String, i32, String> = a.into();
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
        let b: Bed12<String, i32, String, i32, i32, String, Vec<i32>, Vec<i32>> = a.into();
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
        let b: Bed12<String, i32, String, i32, i32, String, Vec<i32>, Vec<i32>> = a.into();
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
        let b: Bed12<String, i32, String, i32, i32, String, Vec<i32>, Vec<i32>> = a.into();
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
        let b: Bed12<String, i32, String, i32, i32, String, String, String> =
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
