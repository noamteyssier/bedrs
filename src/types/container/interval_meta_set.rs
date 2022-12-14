use crate::traits::{Container, IntervalBounds, ValueBounds};
use crate::types::IntervalMeta;
use anyhow::{bail, Result};

/// A collection of [IntervalMeta]
#[derive(Debug, Clone)]
pub struct IntervalMetaSet<T, M>
where
    T: ValueBounds,
    M: Copy,
{
    records: Vec<IntervalMeta<T, M>>,
    is_sorted: bool,
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
    use crate::{
        traits::Container,
        types::{IntervalMeta, IntervalMetaSet},
    };

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
}
