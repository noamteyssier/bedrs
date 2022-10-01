use std::fmt::Debug;

use crate::traits::Container;
use crate::traits::{IntervalBounds, ValueBounds};
use crate::types::Interval;
use anyhow::{bail, Result};

/// A collection of [Interval]
#[derive(Debug, Clone)]
pub struct IntervalSet<T>
where
    T: ValueBounds,
{
    records: Vec<Interval<T>>,
    is_sorted: bool,
}

impl<T> Container<T, Interval<T>> for IntervalSet<T>
where
    Interval<T>: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<Interval<T>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }
    fn records(&self) -> &Vec<Interval<T>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<Interval<T>> {
        &mut self.records
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn set_sorted(&mut self) {
        self.is_sorted = true;
    }
}

impl<T> IntervalSet<T>
where
    T: ValueBounds,
{
    #[must_use]
    pub fn new(records: Vec<Interval<T>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }

    pub fn from_endpoints(starts: &[T], ends: &[T]) -> Result<Self> {
        if starts.len() != ends.len() {
            bail!("Unequal array lengths")
        }
        Ok(Self::from_endpoints_unchecked(starts, ends))
    }

    pub fn from_endpoints_unchecked(starts: &[T], ends: &[T]) -> Self {
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(x, y)| Interval::new(*x, *y))
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
        types::{Interval, IntervalSet},
    };

    #[test]
    fn test_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalSet::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let set = IntervalSet::from_endpoints(&starts, &ends).unwrap();
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_set_init_from_endpoints_unequal() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = IntervalSet::from_endpoints(&starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_interval_set_init_from_endpoints_unequal_unchecked() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        assert_eq!(set.len(), n_intervals);
    }
}
