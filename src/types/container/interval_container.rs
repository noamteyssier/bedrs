use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Container,
};
use anyhow::{bail, Result};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    records: Vec<I>,
    is_sorted: bool,
    max_len: Option<T>,
    _phantom_c: PhantomData<C>,
}
impl<I, C, T> FromIterator<I> for IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn from_iter<It: IntoIterator<Item = I>>(iter: It) -> Self {
        let mut max_len = zero::<T>();
        let records = iter
            .into_iter()
            .map(|iv| {
                max_len = max_len.max(iv.len());
                iv
            })
            .collect();
        let max_len = if max_len == zero::<T>() {
            None
        } else {
            Some(max_len)
        };
        Self {
            records,
            is_sorted: false,
            max_len,
            _phantom_c: PhantomData,
        }
    }
}
impl<I, C, T> Container<C, T, I> for IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            is_sorted: false,
            max_len,
            _phantom_c: PhantomData,
        }
    }
    fn empty() -> Self {
        Self::new(Vec::new())
    }
    fn records(&self) -> &Vec<I> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.records
    }
    fn records_owned(self) -> Vec<I> {
        self.records
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn sorted_mut(&mut self) -> &mut bool {
        &mut self.is_sorted
    }
    fn max_len(&self) -> Option<T> {
        self.max_len
    }
    fn max_len_mut(&mut self) -> &mut Option<T> {
        &mut self.max_len
    }
    fn span(&self) -> Result<I> {
        if self.is_empty() {
            bail!("Cannot get span of empty interval set")
        } else if !self.is_sorted() {
            bail!("Cannot get span of unsorted interval set")
        } else {
            let first = self.records().first().unwrap();
            let last = self.records().last().unwrap();
            if first.chr() != last.chr() {
                bail!("Cannot get span of interval set spanning multiple chromosomes")
            } else {
                let mut iv = I::empty();
                iv.update_chr(first.chr());
                iv.update_start(&first.start());
                iv.update_end(&last.end());
                Ok(iv)
            }
        }
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::Container;
    use crate::{
        Coordinates, GenomicInterval, Interval, NamedInterval, Strand, StrandedGenomicInterval,
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    // --------------------- //
    // Base Interval Testing //
    // --------------------- //

    #[test]
    fn build_interval_container() {
        let records = vec![
            Interval::new(1, 10),
            Interval::new(2, 20),
            Interval::new(3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_base_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_from_iterator() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_base_serialization() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<Interval<usize>, usize, usize> =
            deserialize(&serialized).unwrap();
        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_base_par_sort() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }

    // ------------------------ //
    // Genomic Interval Testing //
    // ------------------------ //

    #[test]
    fn build_genomic_interval_container() {
        let records = vec![
            GenomicInterval::new(1, 1, 10),
            GenomicInterval::new(1, 2, 20),
            GenomicInterval::new(1, 3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_genomic_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_empty_iterator() {
        let records: Vec<GenomicInterval<usize>> = vec![];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), 0);
        assert!(set.max_len().is_none());
        assert!(set.span().is_err());
    }

    #[test]
    fn test_genomic_span() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 100),
            GenomicInterval::new(1, 20, 200),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        assert!(set.span().unwrap().eq(&GenomicInterval::new(1, 10, 200)));
    }

    #[test]
    fn test_genomic_span_errors() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 100),
            GenomicInterval::new(2, 20, 200),
        ];
        let mut set = IntervalContainer::from_iter(intervals);
        match set.span() {
            Err(e) => assert_eq!(e.to_string(), "Cannot get span of unsorted interval set"),
            _ => panic!("Expected error"),
        };
        set.sort();
        match set.span() {
            Err(e) => assert_eq!(
                e.to_string(),
                "Cannot get span of interval set spanning multiple chromosomes"
            ),
            _ => panic!("Expected error"),
        };
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_genomic_serialization() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<GenomicInterval<usize>, usize, usize> =
            deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(&iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_genomic_par_sort() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let mut set = IntervalContainer::new(records.clone());
        set.par_sort();
        for (iv1, iv2) in set.records().iter().zip(records.iter()) {
            assert!(iv1.eq(&iv2));
        }
    }

    // ---------------------- //
    // Named Interval Testing //
    // ---------------------- //

    #[test]
    fn build_named_interval_container() {
        let records = vec![
            NamedInterval::new("chr1", 1, 10),
            NamedInterval::new("chr1", 2, 20),
            NamedInterval::new("chr1", 3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    // ------------------------- //
    // Stranded Interval Testing //
    // ------------------------- //

    #[test]
    fn build_stranded_genomic_interval_container() {
        let records = vec![
            StrandedGenomicInterval::new(1, 1, 10, Strand::Forward),
            StrandedGenomicInterval::new(1, 2, 20, Strand::Forward),
            StrandedGenomicInterval::new(1, 3, 30, Strand::Forward),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_stranded_genomic_init_from_records() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_set_sorted() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let mut set = IntervalContainer::new(records);
        assert_eq!(set.is_sorted(), false);
        set.set_sorted();
        assert_eq!(set.is_sorted(), true);
    }

    #[test]
    fn test_stranded_genomic_set_records_mut() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        let mut set = IntervalContainer::new(records);

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Forward);
        });

        set.records_mut().iter_mut().for_each(|r| {
            r.set_strand(Strand::Reverse);
        });

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Reverse);
        });
    }

    #[test]
    fn test_span_empty() {
        let set: IntervalContainer<StrandedGenomicInterval<u32>, u32, u32> =
            IntervalContainer::new(vec![]);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_unsorted() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        let set = IntervalContainer::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_multiple_chr() {
        let n_intervals = 10;
        let mut records =
            vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        records.push(StrandedGenomicInterval::new(2, 10, 100, Strand::Forward));
        let set = IntervalContainer::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span() {
        let records = vec![
            StrandedGenomicInterval::new(1, 10, 100, Strand::Forward),
            StrandedGenomicInterval::new(1, 1000, 2000, Strand::Forward),
        ];
        let set = IntervalContainer::from_sorted(records).unwrap();
        let span = set.span().unwrap();
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 2000);
    }

    #[test]
    fn test_sort() {
        let records = vec![
            StrandedGenomicInterval::new(1, 1000, 2000, Strand::Reverse),
            StrandedGenomicInterval::new(1, 1000, 2000, Strand::Forward),
            StrandedGenomicInterval::new(1, 1000, 2000, Strand::Unknown),
            StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse),
            StrandedGenomicInterval::new(1, 10, 100, Strand::Forward),
            StrandedGenomicInterval::new(1, 10, 100, Strand::Unknown),
            StrandedGenomicInterval::new(2, 1000, 2000, Strand::Reverse),
            StrandedGenomicInterval::new(2, 1000, 2000, Strand::Forward),
            StrandedGenomicInterval::new(2, 1000, 2000, Strand::Unknown),
            StrandedGenomicInterval::new(2, 10, 100, Strand::Reverse),
            StrandedGenomicInterval::new(2, 10, 100, Strand::Forward),
            StrandedGenomicInterval::new(2, 10, 100, Strand::Unknown),
        ];
        let set = IntervalContainer::from_unsorted(records);
        assert!(set.is_sorted());
        let vec = set.records();
        assert!(vec[0].eq(&StrandedGenomicInterval::new(1, 10, 100, Strand::Forward)));
        assert!(vec[1].eq(&StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse)));
        assert!(vec[2].eq(&StrandedGenomicInterval::new(1, 10, 100, Strand::Unknown)));
        assert!(vec[3].eq(&StrandedGenomicInterval::new(
            1,
            1000,
            2000,
            Strand::Forward
        )));
        assert!(vec[4].eq(&StrandedGenomicInterval::new(
            1,
            1000,
            2000,
            Strand::Reverse
        )));
        assert!(vec[5].eq(&StrandedGenomicInterval::new(
            1,
            1000,
            2000,
            Strand::Unknown
        )));
        assert!(vec[6].eq(&StrandedGenomicInterval::new(2, 10, 100, Strand::Forward)));
        assert!(vec[7].eq(&StrandedGenomicInterval::new(2, 10, 100, Strand::Reverse)));
        assert!(vec[8].eq(&StrandedGenomicInterval::new(2, 10, 100, Strand::Unknown)));
        assert!(vec[9].eq(&StrandedGenomicInterval::new(
            2,
            1000,
            2000,
            Strand::Forward
        )));
        assert!(vec[10].eq(&StrandedGenomicInterval::new(
            2,
            1000,
            2000,
            Strand::Reverse
        )));
        assert!(vec[11].eq(&StrandedGenomicInterval::new(
            2,
            1000,
            2000,
            Strand::Unknown
        )));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<StrandedGenomicInterval<usize>, usize, usize> =
            deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_par_sort() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }
}
