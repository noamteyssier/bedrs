use crate::{
    traits::{container::Merge, Container, Find, IntervalBounds, ValueBounds},
    types::GenomicInterval,
};
use anyhow::{bail, Result};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct GenomicIntervalSet<T> {
    records: Vec<GenomicInterval<T>>,
    is_sorted: bool,
}
impl<T> Container<T, GenomicInterval<T>> for GenomicIntervalSet<T>
where
    GenomicInterval<T>: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<GenomicInterval<T>>) -> Self {
        Self { records, is_sorted: false }
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
}
impl<T> GenomicIntervalSet<T>
where
    T: ValueBounds,
{
    #[must_use]
    pub fn new(records: Vec<GenomicInterval<T>>) -> Self {
        Self { records, is_sorted: false }
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
        Self { records, is_sorted: false }
    }
}
//
impl<T> Merge<T, GenomicInterval<T>> for GenomicIntervalSet<T>
where
    T: ValueBounds,
    GenomicInterval<T>: IntervalBounds<T>,
{
}
impl<T> Find<T, GenomicInterval<T>> for GenomicIntervalSet<T>
where
    T: ValueBounds,
    GenomicInterval<T>: IntervalBounds<T>,
{
    type ContainerType = Self;
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
}
