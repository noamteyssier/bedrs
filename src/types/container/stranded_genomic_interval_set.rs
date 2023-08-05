use crate::{
    traits::{Container, IntervalBounds, ValueBounds},
    types::StrandedGenomicInterval,
    Coordinates, Strand,
};
use anyhow::{bail, Result};
use std::fmt::Debug;

/// A collection of [StrandedGenomicInterval]
#[derive(Debug, Clone)]
pub struct StrandedGenomicIntervalSet<T> {
    records: Vec<StrandedGenomicInterval<T>>,
    is_sorted: bool,
}
impl<T> FromIterator<StrandedGenomicInterval<T>> for StrandedGenomicIntervalSet<T> {
    fn from_iter<I: IntoIterator<Item = StrandedGenomicInterval<T>>>(iter: I) -> Self {
        Self {
            records: iter.into_iter().collect(),
            is_sorted: false,
        }
    }
}
impl<T> Container<T, StrandedGenomicInterval<T>> for StrandedGenomicIntervalSet<T>
where
    StrandedGenomicInterval<T>: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<StrandedGenomicInterval<T>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }
    fn records(&self) -> &Vec<StrandedGenomicInterval<T>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<StrandedGenomicInterval<T>> {
        &mut self.records
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn set_sorted(&mut self) {
        self.is_sorted = true;
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
    /// assert_eq!(span.chr(), 1);
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
                    first.chr(),
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
        Self {
            records,
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
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .zip(strands.iter())
            .map(|(((c, x), y), s)| StrandedGenomicInterval::new(*c, *x, *y, *s))
            .collect();
        Self {
            records,
            is_sorted: false,
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::Container,
        types::{Strand, StrandedGenomicInterval, StrandedGenomicIntervalSet},
    };

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
}