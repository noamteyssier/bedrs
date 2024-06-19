use crate::{
    bed3,
    traits::{ChromBounds, MetaBounds},
    types::{enums::Frame, Score},
    Bed3, Coordinates, Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
use getset::{CopyGetters, Getters, Setters};
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
#[allow(clippy::too_many_arguments)]
#[derive(Debug, Clone, Copy, Default, Coordinates, Getters, Setters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gtf<C, N>
where
    C: ChromBounds,
    N: MetaBounds,
{
    chr: C,
    #[getset(get = "pub", set = "pub")]
    source: N,
    #[getset(get = "pub", set = "pub")]
    feature: N,
    start: i32,
    end: i32,
    #[getset(get_copy = "pub", set = "pub")]
    score: Score,
    strand: Strand,
    #[getset(get_copy = "pub", set = "pub")]
    frame: Frame,
    #[getset(get = "pub", set = "pub")]
    attributes: N,
}

impl<C, N> From<Gtf<C, N>> for Bed3<C>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(record: Gtf<C, N>) -> Self {
        bed3![record.chr, record.start, record.end]
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

        record.set_source("Havana");
        record.set_feature("transcript");
        record.set_score(Score(None));
        record.set_frame(1.into());
        record.set_attributes("");

        assert_eq!(record.source(), &"Havana");
        assert_eq!(record.feature(), &"transcript");
        assert_eq!(record.score(), Score(None));
        assert_eq!(record.frame(), 1.into());
        assert_eq!(record.attributes(), &"");
    }

    #[test]
    fn test_collect() {
        let set: IntervalContainer<Gtf<usize, usize>, _> =
            IntervalContainer::from_iter(vec![Gtf::empty(); 10]);
        assert_eq!(set.len(), 10);
    }

    #[test]
    fn test_into_bed3() {
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
        let bed3: Bed3<_> = record.into();
        assert_eq!(bed3.chr(), &1);
        assert_eq!(bed3.start(), 10);
        assert_eq!(bed3.end(), 30);
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
        let b: Gtf<String, String> = iter.next().unwrap()?;
        assert_eq!(b.chr(), "chr1");
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
