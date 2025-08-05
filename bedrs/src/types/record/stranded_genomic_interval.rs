#[cfg(feature = "htslib")]
use crate::prelude::SetError;
use crate::{
    traits::{ChromBounds, Coordinates, ValueBounds},
    Strand,
};
use bedrs_derive::Coordinates;
use derive_new::new;
#[cfg(feature = "htslib")]
use rust_htslib::bam::{ext::BamRecordExtensions, Record};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "htslib")]
use std::convert::TryFrom;

/// A representation of a Genomic Interval.
///
/// Has three coordinates: `chr`, `start`, and `end`.
/// Has an associated `Strand` which can be either `Forward` or `Reverse`.
/// This is an associated metadata and is not used in comparisons.
///
/// ```
/// use bedrs::{Coordinates, StrandedBed3, Overlap, Strand, StrandedOverlap};
///
/// let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(a.strand(), Some(Strand::Forward));
///
/// let b = StrandedBed3::new(1, 20, 30, Strand::Reverse);
/// assert!(a.overlaps(&b));
/// assert!(!a.stranded_overlaps(&b));
/// ```
#[derive(Debug, Default, Clone, Copy, Coordinates, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrandedBed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    chr: C,
    start: T,
    end: T,
    strand: Strand,
}

/// Implements conversion of a Rust Htslib BAM record to a `StrandedBed3` interval
///
/// If record is unmapped, we return an Error.
/// If record is mapped, we use the struct's functions to populate
/// `chr`, `start`, `end` and `strand`.
///
/// ```
/// use bedrs::{Coordinates, Strand, StrandedBed3, traits::SetError};
/// use rust_htslib::bam::Record;
/// use rust_htslib::bam::record::{Cigar, CigarString};
///
/// // create a forward read with some data
/// let mut r = Record::new();
/// r.set_tid(9);
/// r.set_pos(1200);
/// r.unset_unmapped();
/// r.set(&vec![b'r',b'e',b'a',b'd'], Some(&CigarString(vec![Cigar::Match(100)])),
///       &vec![ b'A' as u8; 100], &vec![255 as u8; 100]);
///
/// // ensure we recover it after conversion
/// let a = StrandedBed3::try_from(r)?;
/// assert_eq!(*a.chr(), 9);
/// assert_eq!(a.start(), 1200);
/// assert_eq!(a.end(), 1300);
/// assert_eq!(a.strand(), Some(Strand::Forward));
///
/// // create a reverse read with some data.
/// // mark it supplementary, but this shouldn't make a difference.
/// let mut s = Record::new();
/// s.set_tid(5);
/// s.set_pos(200);
/// s.unset_unmapped();
/// s.set_reverse();
/// s.set_supplementary();
/// s.set(&vec![b'r',b'e',b'a',b'd'], Some(&CigarString(vec![Cigar::Match(150)])),
///       &vec![ b'A' as u8; 150], &vec![255 as u8; 150]);
///
/// // ensure we recover it after conversion
/// let b = StrandedBed3::try_from(s)?;
/// assert_eq!(*b.chr(), 5);
/// assert_eq!(b.start(), 200);
/// assert_eq!(b.end(), 350);
/// assert_eq!(b.strand(), Some(Strand::Reverse));
/// # Ok::<(), SetError>(())
/// ```
///
/// ```should_panic
/// use bedrs::{Coordinates, Strand, StrandedBed3, traits::SetError};
/// use rust_htslib::bam::Record;
/// use rust_htslib::bam::record::{Cigar, CigarString};
///
/// // following is an unmapped read, so we should panic
/// let mut t = Record::new();
/// t.set_tid(5);
/// t.set_pos(200);
/// t.set(&vec![b'r',b'e',b'a',b'd'], Some(&CigarString(vec![Cigar::Match(150)])),
///       &vec![ b'A' as u8; 150], &vec![255 as u8; 150]);
///
/// let c = StrandedBed3::try_from(t)?;
/// # Ok::<(), SetError>(())
/// ```
///
/// ```
/// // Load a BAM file and convert records to StrandedBed3.
/// use bedrs::prelude::*;
/// use rust_htslib::{bam, bam::Read};
///
/// let mut bam = bam::Reader::from_path(&"examples/sample.sorted.bam").unwrap();
/// for r in bam.records() {
///     let record = r.unwrap();
///     let a = StrandedBed3::try_from(record).unwrap();
/// }
/// ```
#[cfg(feature = "htslib")]
impl TryFrom<Record> for StrandedBed3<i32, i64> {
    type Error = SetError;

    fn try_from(value: Record) -> Result<Self, Self::Error> {
        if value.is_unmapped() {
            Err(SetError::EmptySet)
        } else {
            Ok(StrandedBed3 {
                chr: value.tid(),
                start: value.pos(),
                end: value.reference_end(),
                strand: if value.is_reverse() {
                    Strand::Reverse
                } else {
                    Strand::Forward
                },
            })
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::Coordinates,
        types::{Strand, StrandedBed3},
        Subtract,
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 90, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedBed3::new(2, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let b = StrandedBed3::new(2, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = StrandedBed3::new(2, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(2, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    fn test_set_strand() {
        let mut a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.strand(), Some(Strand::Forward));
        a.update_strand(Some(Strand::Reverse));
        assert_eq!(a.strand(), Some(Strand::Reverse));
    }

    #[test]
    fn test_subtraction_a() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Forward);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_b() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_c() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b = StrandedBed3::new(1, 10, 100, Strand::Reverse);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand().unwrap(), Strand::Reverse);
    }

    #[test]
    fn test_from() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let b: StrandedBed3<_, _> = Coordinates::from(&a);
        assert!(a.eq(&b));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn stranded_genomic_interval_serde() {
        let a = StrandedBed3::new(1, 5, 100, Strand::Reverse);
        let serialized = serialize(&a).unwrap();
        let deserialized: StrandedBed3<_, _> = deserialize(&serialized).unwrap();
        assert!(a.eq(&deserialized));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_some());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = StrandedBed3::new(1, 10, 100, Strand::Forward);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
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
        let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
        wtr.serialize(a)?;
        let result = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(result, "1,20,30,+\n");
        Ok(())
    }

    #[test]
    fn test_csv_deserialization() -> Result<()> {
        let a = "1,20,30,+\n";
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let b: StrandedBed3<i32, i32> = iter.next().unwrap()?;
        assert_eq!(b.chr(), &1);
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.strand(), Some(Strand::Forward));
        Ok(())
    }
}

#[cfg(feature = "htslib")]
#[cfg(test)]
mod htslib_testing {
    use super::*;
    use crate::{Coordinates, Intersect, Strand, StrandedBed3};
    use anyhow::Result;
    use rust_htslib::{bam, bam::Read};

    #[test]
    fn test_bam_intersect() -> Result<()> {
        let a = StrandedBed3::new(1, 19, 70, Strand::Forward);
        let mut bam = bam::Reader::from_path(&"examples/sample.sorted.bam").unwrap();
        for r in bam.records() {
            let record = r?;
            let b = StrandedBed3::try_from(record)?;
            if let Some(c) = a.stranded_intersect(&b) {
                assert_eq!(c.start(), 19);
                assert_eq!(c.end(), 59);
                assert_eq!(c.strand(), Some(Strand::Forward));
            }
        }
        Ok(())
    }
}
