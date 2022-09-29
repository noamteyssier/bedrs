use crate::traits::Container;
use crate::types::IntervalMeta;
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct IntervalMetaSet<T, M> {
    records: Vec<IntervalMeta<T, M>>,
}

impl<T, M> Container<T, IntervalMeta<T, M>> for IntervalMetaSet<T, M> {
    fn records(&self) -> &Vec<IntervalMeta<T, M>> {
        &self.records
    }
}

impl<T, M> IntervalMetaSet<T, M>
where
    T: Copy,
{
    pub fn new(records: Vec<IntervalMeta<T, M>>) -> Self {
        Self { records }
    }

    pub fn from_endpoints(starts: &[T], ends: &[T]) -> Result<Self> {
        if starts.len() != ends.len() {
            bail!("Unequal array lengths")
        } else {
            Ok(Self::from_endpoints_unchecked(starts, ends))
        }
    }

    pub fn from_endpoints_unchecked(starts: &[T], ends: &[T]) -> Self {
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(x, y)| IntervalMeta::new(*x, *y, None))
            .collect();
        Self { records }
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
