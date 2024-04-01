use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::{enums::Frame, Score},
    Bed3, Coordinates, Strand,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// This is a representation of the [GTF file format](https://www.ensembl.org/info/website/upload/gff.html)
/// which is identical to the GFF version 2 format.
///
/// The GTF format is a tab-delimited file that must have the following columns:
/// 1. seqname - The name of the sequence. Must be a chromosome or scaffold.
/// 2. source - The program that generated this feature.
/// 3. feature - The name of this type of feature. Some examples of standard features are "CDS" and "exon".
/// 4. start - The starting position of the feature in the sequence. The first base is numbered 1.
/// 5. end - The ending position of the feature (inclusive).
/// 6. score - A score between 0 and 1000. If there is no score, use ".".
/// 7. strand - The strand on which the feature is located. Valid values include '+', '-', or '.' (for don't know/don't care).
/// 8. frame - The frame of the feature. It must be a number between 0-2 or "." if the feature is not a coding exon.
/// 9. attributes - A semicolon-separated list of tag-value pairs, providing additional information about each feature.
///
/// # Usage
///
/// ```
/// use bedrs::{Gtf, Coordinates, Strand, Score};
///
/// let record = Gtf::new(
///     "scaffold_1",
///     "ENSEMBL",
///     "gene",
///     1000,
///     4000,
///     Score(None),
///     Strand::Forward,
///     1.into(),
///     "gene AP2S1; transcript AP2S1_201;"
/// );
/// assert_eq!(record.chr(), &"scaffold_1");
/// assert_eq!(record.start(), 1000);
/// assert_eq!(record.end(), 4000);
/// assert_eq!(record.strand(), Some(Strand::Forward));
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gtf<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    seqname: C,
    source: N,
    feature: N,
    start: T,
    end: T,
    score: Score,
    strand: Strand,
    frame: Frame,
    attributes: N,
}
impl<C, T, N> Coordinates<C, T> for Gtf<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn empty() -> Self {
        Self::default()
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.seqname
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
        self.seqname = val.clone();
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        self.strand = strand.unwrap_or_default();
    }
    fn from<Iv: Coordinates<C, T>>(other: &Iv) -> Self {
        Self {
            seqname: other.chr().clone(),
            source: N::default(),
            feature: N::default(),
            start: other.start(),
            end: other.end(),
            score: Score::default(),
            strand: other.strand().unwrap_or_default(),
            frame: Frame::default(),
            attributes: N::default(),
        }
    }
}
impl<'a, C, T, N> Coordinates<C, T> for &'a Gtf<C, T, N>
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
        &self.seqname
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
impl<'a, C, T, N> Coordinates<C, T> for &'a mut Gtf<C, T, N>
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
        &self.seqname
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
        self.seqname = val.clone();
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        self.strand = strand.unwrap_or_default();
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
    }
}
impl<C, T, N> Gtf<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        seqname: C,
        source: N,
        feature: N,
        start: T,
        end: T,
        score: Score,
        strand: Strand,
        frame: Frame,
        attributes: N,
    ) -> Self {
        Self {
            seqname,
            source,
            feature,
            start,
            end,
            score,
            strand,
            frame,
            attributes,
        }
    }
    pub fn seqname(&self) -> &C {
        &self.seqname
    }
    pub fn source(&self) -> &N {
        &self.source
    }
    pub fn feature(&self) -> &N {
        &self.feature
    }
    pub fn score(&self) -> Score {
        self.score
    }
    pub fn frame(&self) -> Frame {
        self.frame
    }
    pub fn attributes(&self) -> &N {
        &self.attributes
    }
    pub fn update_seqname(&mut self, val: &C) {
        self.update_chr(val);
    }
    pub fn update_source(&mut self, val: &N) {
        self.source = val.clone();
    }
    pub fn update_feature(&mut self, val: &N) {
        self.feature = val.clone();
    }
    pub fn update_score(&mut self, val: Score) {
        self.score = val;
    }
    pub fn update_frame(&mut self, val: Frame) {
        self.frame = val;
    }
    pub fn update_attributes(&mut self, val: &N) {
        self.attributes = val.clone();
    }
}

impl<C, T, N> From<Gtf<C, T, N>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(record: Gtf<C, T, N>) -> Self {
        Self::new(record.seqname, record.start, record.end)
    }
}

#[cfg(test)]
mod testing {
    use crate::IntervalContainer;

    use super::*;

    #[test]
    fn test_init() {
        let record = Gtf::new(
            1,
            "Ensembl",
            "gene",
            10,
            30,
            11.1.into(),
            Strand::Reverse,
            0.into(),
            "some_attr",
        );
        assert_eq!(record.chr(), &1);
        assert_eq!(record.start(), 10);
        assert_eq!(record.end(), 30);
        assert_eq!(record.strand(), Some(Strand::Reverse));
        assert_eq!(record.source(), &"Ensembl");
        assert_eq!(record.feature(), &"gene");
        assert_eq!(record.score(), 11.1.into());
        assert_eq!(record.frame(), 0.into());
        assert_eq!(record.attributes(), &"some_attr");
    }

    #[test]
    fn test_update() {
        let mut record = Gtf::new(
            1,
            "Ensembl",
            "gene",
            10,
            30,
            11.1.into(),
            Strand::Reverse,
            0.into(),
            "some_attr",
        );
        assert_eq!(record.chr(), &1);
        assert_eq!(record.start(), 10);
        assert_eq!(record.end(), 30);
        assert_eq!(record.strand(), Some(Strand::Reverse));
        assert_eq!(record.source(), &"Ensembl");
        assert_eq!(record.feature(), &"gene");
        assert_eq!(record.score(), 11.1.into());
        assert_eq!(record.frame(), 0.into());
        assert_eq!(record.attributes(), &"some_attr");

        record.update_seqname(&2);
        record.update_source(&"Havana");
        record.update_feature(&"transcript");
        record.update_score(Score(None));
        record.update_frame(1.into());
        record.update_attributes(&"");

        assert_eq!(record.seqname(), &2);
        assert_eq!(record.source(), &"Havana");
        assert_eq!(record.feature(), &"transcript");
        assert_eq!(record.score(), Score(None));
        assert_eq!(record.frame(), 1.into());
        assert_eq!(record.attributes(), &"");
    }

    #[test]
    fn test_collect() {
        let set: IntervalContainer<Gtf<usize, usize, usize>, _, _> =
            IntervalContainer::from_iter(vec![Gtf::empty(); 10]);
        assert_eq!(set.len(), 10);
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
        let a = Gtf::new(
            "chr1",
            "Ensembl",
            "gene",
            20,
            30,
            Score(None),
            Strand::Unknown,
            0.into(),
            "metadata",
        );
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "chr1,Ensembl,gene,20,30,.,.,0,metadata\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "chr1,Ensembl,gene,20,30,.,.,0,metadata\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: Gtf<String, i32, String> = iter.next().unwrap()?;
        assert_eq!(b.seqname(), "chr1");
        assert_eq!(b.source(), "Ensembl");
        assert_eq!(b.feature(), "gene");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.score(), Score(None));
        assert_eq!(b.frame(), 0.into());
        assert_eq!(b.attributes(), "metadata");
        Ok(())
    }
}
