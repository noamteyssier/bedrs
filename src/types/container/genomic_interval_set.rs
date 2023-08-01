use crate::{
    traits::{Container, IntervalBounds, ValueBounds},
    types::GenomicInterval, Coordinates,
};
use anyhow::{bail, Result};
use std::fmt::Debug;

/// A collection of [GenomicInterval]
#[derive(Debug, Clone)]
pub struct GenomicIntervalSet<T> {
    records: Vec<GenomicInterval<T>>,
    is_sorted: bool,
}
impl<T> FromIterator<GenomicInterval<T>> for GenomicIntervalSet<T> {
    fn from_iter<I: IntoIterator<Item = GenomicInterval<T>>>(iter: I) -> Self {
        Self {
            records: iter.into_iter().collect(),
            is_sorted: false,
        }
    }
}
impl<T> Container<T, GenomicInterval<T>> for GenomicIntervalSet<T>
where
    GenomicInterval<T>: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<GenomicInterval<T>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }
    fn records(&self) -> &Vec<GenomicInterval<T>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<GenomicInterval<T>> {
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
    ///     types::{GenomicInterval, GenomicIntervalSet},
    /// };
    ///
    /// let mut ivs = GenomicIntervalSet::from_iter(vec![
    ///     GenomicInterval::new(1, 10, 100),
    ///     GenomicInterval::new(1, 200, 300),
    ///     GenomicInterval::new(1, 400, 500),
    /// ]);
    /// ivs.set_sorted();
    ///
    /// let span = ivs.span().unwrap();
    /// assert_eq!(span.chr(), 1);
    /// assert_eq!(span.start(), 10);
    /// assert_eq!(span.end(), 500);
    /// ```
    fn span(&self) -> Result<GenomicInterval<T>> {
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
                let iv = GenomicInterval::new(first.chr(), first.start(), last.end());
                Ok(iv)
            }
        }
    }
}
impl<T> GenomicIntervalSet<T>
where
    T: ValueBounds,
{
    #[must_use]
    pub fn new(records: Vec<GenomicInterval<T>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }

    pub fn from_endpoints(chrs: &[T], starts: &[T], ends: &[T]) -> Result<Self> {
        if (chrs.len() == starts.len()) && (starts.len() == ends.len()) {
            Ok(Self::from_endpoints_unchecked(chrs, starts, ends))
        } else {
            bail!("Unequal array lengths")
        }
    }

    pub fn from_endpoints_unchecked(chrs: &[T], starts: &[T], ends: &[T]) -> Self {
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .map(|((c, x), y)| GenomicInterval::new(*c, *x, *y))
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
        types::{GenomicInterval, GenomicIntervalSet},
    };

    #[test]
    fn test_genomic_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = GenomicIntervalSet::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let set = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends).unwrap();
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_chr() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals + 3];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let set = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_starts() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals + 3];
        let ends = vec![100; n_intervals];
        let set = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_ends() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_genomic_interval_set_init_from_endpoints_unequal_unchecked() {
        let n_intervals = 10;
        let chrs = vec![1; n_intervals];
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = GenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_from_iterator() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = GenomicIntervalSet::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }
}
