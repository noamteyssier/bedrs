use crate::{
    traits::{ChromBounds, Container, IntervalBounds, ValueBounds},
    types::StrandedGenomicInterval,
    Coordinates, Strand,
};
use anyhow::{bail, Result};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A collection of [StrandedGenomicInterval]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrandedGenomicIntervalSet<T> {
    records: Vec<StrandedGenomicInterval<T>>,
    max_len: Option<T>,
    is_sorted: bool,
}
impl<T> FromIterator<StrandedGenomicInterval<T>> for StrandedGenomicIntervalSet<T>
where
    T: ValueBounds,
{
    fn from_iter<I: IntoIterator<Item = StrandedGenomicInterval<T>>>(iter: I) -> Self {
        let mut max_len = zero::<T>();
        let records = iter
            .into_iter()
            .map(|interval| {
                max_len = max_len.max(interval.len());
                interval
            })
            .collect();
        let max_len = if max_len == zero::<T>() {
            None
        } else {
            Some(max_len)
        };
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }
}
impl<T> Container<T, T, StrandedGenomicInterval<T>> for StrandedGenomicIntervalSet<T>
where
    StrandedGenomicInterval<T>: IntervalBounds<T, T>,
    T: ValueBounds + ChromBounds,
{
    fn new(records: Vec<StrandedGenomicInterval<T>>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }
    fn records(&self) -> &Vec<StrandedGenomicInterval<T>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<StrandedGenomicInterval<T>> {
        &mut self.records
    }
    fn records_owned(self) -> Vec<StrandedGenomicInterval<T>> {
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

    /// Get the span of the interval set
    ///
    /// # Errors
    /// * If the interval set is empty
    /// * If the interval set is not sorted
    /// * If the interval set spans multiple chromosomes
    ///
    /// # Examples
    /// ```
    /// use bedrs::{
    ///     traits::{Container, Coordinates},
    ///     types::{StrandedGenomicInterval, StrandedGenomicIntervalSet, Strand},
    /// };
    ///
    /// let mut ivs = StrandedGenomicIntervalSet::from_iter(vec![
    ///     StrandedGenomicInterval::new(1, 10, 100, Strand::Forward),
    ///     StrandedGenomicInterval::new(1, 200, 300, Strand::Forward),
    ///     StrandedGenomicInterval::new(1, 400, 500, Strand::Reverse),
    /// ]);
    /// ivs.set_sorted();
    ///
    /// let span = ivs.span().unwrap();
    /// assert_eq!(*span.chr(), 1);
    /// assert_eq!(span.start(), 10);
    /// assert_eq!(span.end(), 500);
    /// assert_eq!(span.strand(), Strand::Unknown); // Strand is arbitrary in span
    /// ```
    fn span(&self) -> Result<StrandedGenomicInterval<T>> {
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
                let iv = StrandedGenomicInterval::new(
                    *first.chr(),
                    first.start(),
                    last.end(),
                    Strand::Unknown,
                );
                Ok(iv)
            }
        }
    }
}
impl<T> StrandedGenomicIntervalSet<T>
where
    T: ValueBounds,
{
    #[must_use]
    pub fn new(records: Vec<StrandedGenomicInterval<T>>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }

    pub fn from_endpoints(
        chrs: &[T],
        starts: &[T],
        ends: &[T],
        strands: &[Strand],
    ) -> Result<Self> {
        if (chrs.len() == starts.len())
            && (starts.len() == ends.len())
            && (ends.len() == strands.len())
        {
            Ok(Self::from_endpoints_unchecked(chrs, starts, ends, strands))
        } else {
            bail!("Unequal array lengths")
        }
    }

    pub fn from_endpoints_unchecked(
        chrs: &[T],
        starts: &[T],
        ends: &[T],
        strands: &[Strand],
    ) -> Self {
        let mut max_len = zero::<T>();
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .zip(strands.iter())
            .map(|(((c, x), y), s)| StrandedGenomicInterval::new(*c, *x, *y, *s))
            .map(|interval| {
                max_len = max_len.max(interval.len());
                interval
            })
            .collect();
        let max_len = if max_len == zero::<T>() {
            None
        } else {
            Some(max_len)
        };
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::Container,
        types::{Strand, StrandedGenomicInterval, StrandedGenomicIntervalSet},
        Coordinates,
    };

    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    #[test]
    fn test_genomic_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = StrandedGenomicIntervalSet::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let strands = vec![Strand::Reverse; n_intervals];
        let set =
            StrandedGenomicIntervalSet::from_endpoints(&chrs, &starts, &ends, &strands).unwrap();
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_chr() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals + 3];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let strands = vec![Strand::Forward; n_intervals];
        let set = StrandedGenomicIntervalSet::from_endpoints(&chrs, &starts, &ends, &strands);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_starts() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals + 3];
        let ends = vec![100; n_intervals];
        let strands = vec![Strand::Forward; n_intervals];
        let set = StrandedGenomicIntervalSet::from_endpoints(&chrs, &starts, &ends, &strands);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_ends() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let strands = vec![Strand::Forward; n_intervals];
        let set = StrandedGenomicIntervalSet::from_endpoints(&chrs, &starts, &ends, &strands);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_unchecked() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let strands = vec![Strand::Forward; n_intervals];
        let set =
            StrandedGenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends, &strands);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_from_iterator() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = StrandedGenomicIntervalSet::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_new() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = StrandedGenomicIntervalSet::new(records);
        assert_eq!(set.len(), n_intervals);
        assert_eq!(set.is_sorted(), false);
    }

    #[test]
    fn test_set_sorted() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let mut set = StrandedGenomicIntervalSet::new(records);
        assert_eq!(set.is_sorted(), false);
        set.set_sorted();
        assert_eq!(set.is_sorted(), true);
    }

    #[test]
    fn test_records_mut() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        let mut set = StrandedGenomicIntervalSet::new(records);

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand(), Strand::Forward);
        });

        set.records_mut().iter_mut().for_each(|r| {
            r.set_strand(Strand::Reverse);
        });

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand(), Strand::Reverse);
        });
    }

    #[test]
    fn test_span_empty() {
        let set = StrandedGenomicIntervalSet::<u32>::new(vec![]);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_unsorted() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        let set = StrandedGenomicIntervalSet::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_multiple_chr() {
        let n_intervals = 10;
        let mut records =
            vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Forward); n_intervals];
        records.push(StrandedGenomicInterval::new(2, 10, 100, Strand::Forward));
        let set = StrandedGenomicIntervalSet::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span() {
        let records = vec![
            StrandedGenomicInterval::new(1, 10, 100, Strand::Forward),
            StrandedGenomicInterval::new(1, 1000, 2000, Strand::Forward),
        ];
        let set = StrandedGenomicIntervalSet::from_sorted(records).unwrap();
        let span = set.span().unwrap();
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 2000);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = StrandedGenomicIntervalSet::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: StrandedGenomicIntervalSet<usize> = deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }
}
