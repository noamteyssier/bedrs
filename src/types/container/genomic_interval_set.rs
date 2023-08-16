use crate::{
    traits::{ChromBounds, Container, IntervalBounds, ValueBounds},
    types::GenomicInterval,
    Coordinates,
};
use anyhow::{bail, Result};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A collection of [GenomicInterval]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenomicIntervalSet<T> {
    records: Vec<GenomicInterval<T>>,
    is_sorted: bool,
    max_len: Option<T>,
}
impl<T> FromIterator<GenomicInterval<T>> for GenomicIntervalSet<T>
where
    T: ValueBounds,
{
    fn from_iter<I: IntoIterator<Item = GenomicInterval<T>>>(iter: I) -> Self {
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
impl<T> Container<T, T, GenomicInterval<T>> for GenomicIntervalSet<T>
where
    GenomicInterval<T>: IntervalBounds<T, T>,
    T: ValueBounds + ChromBounds,
{
    fn new(records: Vec<GenomicInterval<T>>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
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
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
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
        let mut max_len = zero::<T>();
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .map(|((c, x), y)| GenomicInterval::new(*c, *x, *y))
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
        types::{GenomicInterval, GenomicIntervalSet},
    };
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

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

    #[test]
    fn test_from_empty_iterator() {
        let records: Vec<GenomicInterval<usize>> = vec![];
        let set = GenomicIntervalSet::from_iter(records);
        assert_eq!(set.len(), 0);
        assert!(set.max_len().is_none());
        assert!(set.span().is_err());
    }

    #[test]
    fn test_span() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 100),
            GenomicInterval::new(1, 20, 200),
        ];
        let set = GenomicIntervalSet::from_sorted(intervals).unwrap();
        assert_eq!(set.span().unwrap(), GenomicInterval::new(1, 10, 200));
    }

    #[test]
    fn test_span_errors() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 100),
            GenomicInterval::new(2, 20, 200),
        ];
        let mut set = GenomicIntervalSet::from_iter(intervals);
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
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![GenomicInterval::new(1, 10, 100); n_intervals];
        let set = GenomicIntervalSet::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: GenomicIntervalSet<usize> = deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(&iv2));
        }
    }
}
