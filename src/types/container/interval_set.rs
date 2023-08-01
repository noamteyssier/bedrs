use std::fmt::Debug;

use crate::traits::Container;
use crate::traits::{IntervalBounds, ValueBounds};
use crate::types::Interval;
use crate::Coordinates;
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

impl<T> FromIterator<Interval<T>> for IntervalSet<T>
where
    T: ValueBounds,
{
    fn from_iter<I: IntoIterator<Item = Interval<T>>>(iter: I) -> Self {
        Self {
            records: iter.into_iter().collect(),
            is_sorted: false,
        }
    }
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

    /// Get the span of the interval set
    ///
    /// # Errors
    /// * If the interval set is empty
    /// * If the interval set is not sorted
    ///
    /// # Examples
    /// ```
    /// use bedrs::{
    ///     traits::{Container, Coordinates},
    ///     types::{Interval, IntervalSet},
    /// };
    ///
    /// let mut ivs = IntervalSet::from_iter(vec![
    ///     Interval::new(1, 10),
    ///     Interval::new(2, 20),
    ///     Interval::new(3, 30),
    ///     Interval::new(4, 40),
    ///     Interval::new(5, 50),
    /// ]);
    /// ivs.set_sorted();
    ///
    /// let span = ivs.span().unwrap();
    /// assert_eq!(span.start(), 1);
    /// assert_eq!(span.end(), 50);
    fn span(&self) -> Result<Interval<T>> {
        if self.records.is_empty() {
            bail!("Interval set is empty")
        } else if !self.is_sorted() {
            bail!("Cannot get span of unsorted interval set")
        } else {
            let first = self.records.first().unwrap();
            let last = self.records.last().unwrap();
            let iv = Interval::new(first.start(), last.end());
            Ok(iv)
        }
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

    #[test]
    fn test_from_iterator() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalSet::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }
}
