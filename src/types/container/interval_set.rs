use crate::traits::Container;
use crate::traits::{ChromBounds, IntervalBounds, ValueBounds};
use crate::types::Interval;
use crate::Coordinates;
use anyhow::{bail, Result};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A collection of [Interval]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntervalSet<T>
where
    T: ValueBounds,
{
    records: Vec<Interval<T>>,
    max_len: Option<T>,
    is_sorted: bool,
}

impl<T> FromIterator<Interval<T>> for IntervalSet<T>
where
    T: ValueBounds,
{
    fn from_iter<I: IntoIterator<Item = Interval<T>>>(iter: I) -> Self {
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

impl<T> Container<T, T, Interval<T>> for IntervalSet<T>
where
    Interval<T>: IntervalBounds<T, T>,
    T: ValueBounds + ChromBounds,
{
    fn new(records: Vec<Interval<T>>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
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
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
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
        let mut max_len = zero::<T>();
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(x, y)| Interval::new(*x, *y))
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
    #[cfg(feature = "serde")]
    use crate::traits::Coordinates;
    use crate::{
        traits::Container,
        types::{Interval, IntervalSet},
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

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

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalSet::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalSet<usize> = deserialize(&serialized).unwrap();
        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }
}
