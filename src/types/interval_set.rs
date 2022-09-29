use anyhow::{Result, bail};
use super::Interval;

#[derive(Debug, Clone)]
pub struct IntervalSet<T, M> 
{
    records: Vec<Interval<T, M>>
}
impl<T, M> IntervalSet<T, M> 
where T: Copy
{

    pub fn new(records: Vec<Interval<T, M>>) -> Self {
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
            .map(|(x, y)| Interval::new(*x, *y, None))
            .collect();
        Self{records}
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

#[cfg(test)]
mod testing {
    use crate::types::{Interval, IntervalSet};

    #[test]
    fn test_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100, None); n_intervals];
        let set = IntervalSet::<usize, usize>::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals];
        let set = IntervalSet::<usize, usize>::from_endpoints(&starts, &ends).unwrap();
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_interval_set_init_from_endpoints_unequal() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals+3];
        let set = IntervalSet::<usize, usize>::from_endpoints(&starts, &ends);
        assert!(set.is_err());
    }

    #[test]
    fn test_interval_set_init_from_endpoints_unequal_unchecked() {
        let n_intervals = 10;
        let starts = vec![10; n_intervals];
        let ends = vec![100; n_intervals+3];
        let set = IntervalSet::<usize, usize>::from_endpoints_unchecked(&starts, &ends);
        assert_eq!(set.len(), n_intervals);
    }
}
