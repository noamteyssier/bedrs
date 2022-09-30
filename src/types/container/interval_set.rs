use std::fmt::Debug;

use crate::traits::{ValueBounds, IntervalBounds, Find};
use crate::traits::{container::Merge, Container};
use crate::types::Interval;
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct IntervalSet<T>
where
    T: Copy + Default,
{
    records: Vec<Interval<T>>,
}

impl<T> Container<T, Interval<T>> for IntervalSet<T>
where
    Interval<T>: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<Interval<T>>) -> Self {
        Self { records }
    }
    fn records(&self) -> &Vec<Interval<T>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<Interval<T>> {
        &mut self.records
    }
}

impl<T> Merge<T, Interval<T>> for IntervalSet<T>
where
    T: ValueBounds,
    Interval<T>: IntervalBounds<T>,
{
}

impl<T> Find<T, Interval<T>> for IntervalSet<T>
where
    T: ValueBounds,
    Interval<T>: IntervalBounds<T>,
{
    type ContainerType = Self;
}

impl<T> IntervalSet<T>
where
    T: Copy + Default,
{
    #[must_use]
    pub fn new(records: Vec<Interval<T>>) -> Self {
        Self { records }
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
        Self { records }
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
