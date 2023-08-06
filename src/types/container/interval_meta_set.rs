use crate::traits::{Container, IntervalBounds, ValueBounds};
use crate::types::IntervalMeta;
use crate::Coordinates;
use anyhow::{bail, Result};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A collection of [IntervalMeta]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntervalMetaSet<T, M>
where
    T: ValueBounds,
    M: Copy,
{
    records: Vec<IntervalMeta<T, M>>,
    is_sorted: bool,
}

impl<T, M> FromIterator<IntervalMeta<T, M>> for IntervalMetaSet<T, M>
where
    T: ValueBounds,
    M: Copy,
{
    fn from_iter<I: IntoIterator<Item = IntervalMeta<T, M>>>(iter: I) -> Self {
        Self {
            records: iter.into_iter().collect(),
            is_sorted: false,
        }
    }
}

impl<T, M> Container<T, IntervalMeta<T, M>> for IntervalMetaSet<T, M>
where
    IntervalMeta<T, M>: IntervalBounds<T>,
    T: ValueBounds,
    M: Copy,
{
    fn new(records: Vec<IntervalMeta<T, M>>) -> Self {
        Self {
            records,
            is_sorted: false,
        }
    }
    fn records(&self) -> &Vec<IntervalMeta<T, M>> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<IntervalMeta<T, M>> {
        &mut self.records
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn set_sorted(&mut self) {
        self.is_sorted = true;
    }

    /// Get the span of the interval set
    /// Does not copy the meta field of the first and last interval
    ///
    /// # Errors
    /// * If the interval set is empty
    /// * If the interval set is not sorted
    ///
    /// # Examples
    /// ```
    /// use bedrs::{
    ///    traits::{Container, Coordinates},
    ///    types::{IntervalMeta, IntervalMetaSet},
    /// };
    ///
    /// let mut ivs = IntervalMetaSet::from_iter(vec![
    ///     IntervalMeta::new(1, 10, Some(1)),
    ///     IntervalMeta::new(2, 20, Some(2)),
    ///     IntervalMeta::new(3, 30, Some(3)),
    ///     IntervalMeta::new(4, 40, Some(4)),
    ///     IntervalMeta::new(5, 50, Some(5)),
    /// ]);
    /// ivs.set_sorted();
    ///
    /// let span = ivs.span().unwrap();
    /// assert_eq!(span.start(), 1);
    /// assert_eq!(span.end(), 50);
    /// assert!(span.metadata().is_none());
    /// ```
    fn span(&self) -> Result<IntervalMeta<T, M>> {
        if self.is_empty() {
            bail!("Cannot get span of empty interval set")
        } else if !self.is_sorted() {
            bail!("Cannot get span of unsorted interval set")
        } else {
            let first = self.records().first().unwrap();
            let last = self.records().last().unwrap();
            let iv = IntervalMeta::new(first.start(), last.end(), None);
            Ok(iv)
        }
    }
}

impl<T, M> IntervalMetaSet<T, M>
where
    T: ValueBounds,
    M: Copy,
{
    #[must_use]
    pub fn new(records: Vec<IntervalMeta<T, M>>) -> Self {
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
            .map(|(x, y)| IntervalMeta::new(*x, *y, None))
            .collect();
        Self {
            records,
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
        types::{IntervalMeta, IntervalMetaSet},
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    #[test]
    fn test_interval_meta_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![IntervalMeta::new(10, 100, None); n_intervals];
        let set = IntervalMetaSet::<usize, usize>::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_meta_set_init_from_endpoints() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let set = IntervalMetaSet::<usize, usize>::from_endpoints(&starts, &ends).unwrap();
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_meta_set_init_from_endpoints_unequal() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = IntervalMetaSet::<usize, usize>::from_endpoints(&starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_interval_meta_set_init_from_endpoints_unequal_unchecked() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals + 3];
        let set = IntervalMetaSet::<usize, usize>::from_endpoints_unchecked(&starts, &ends);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_from_iterator() {
        let n_intervals = 10;
        let records = vec![IntervalMeta::new(10, 100, None); n_intervals];
        let set: IntervalMetaSet<usize, usize> = IntervalMetaSet::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![IntervalMeta::new(10, 100, None); n_intervals];
        let set: IntervalMetaSet<usize, usize> = IntervalMetaSet::from_iter(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalMetaSet<usize, usize> = deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }
}
