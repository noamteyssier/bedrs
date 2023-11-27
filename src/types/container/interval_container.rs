use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Container,
};
use anyhow::{bail, Result};
use num_traits::zero;
use std::marker::PhantomData;

pub struct IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    records: Vec<I>,
    is_sorted: bool,
    max_len: Option<T>,
    _phantom_c: PhantomData<C>,
}
impl<I, C, T> FromIterator<I> for IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn from_iter<It: IntoIterator<Item = I>>(iter: It) -> Self {
        let mut max_len = zero::<T>();
        let records = iter
            .into_iter()
            .map(|iv| {
                max_len = max_len.max(iv.len());
                iv
            })
            .collect();
        let max_len = if max_len == zero::<T>() {
            None
        } else {
            Some(max_len)
        };
        Self {
            records,
            is_sorted: false,
            max_len,
            _phantom_c: PhantomData,
        }
    }
}
impl<I, C, T> Container<C, T, I> for IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            is_sorted: false,
            max_len,
            _phantom_c: PhantomData,
        }
    }
    fn empty() -> Self {
        Self::new(Vec::new())
    }
    fn records(&self) -> &Vec<I> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.records
    }
    fn records_owned(self) -> Vec<I> {
        self.records
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
    fn span(&self) -> Result<I> {
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
                let mut iv = I::empty();
                iv.update_chr(first.chr());
                iv.update_start(&first.start());
                iv.update_end(&last.end());
                Ok(iv)
            }
        }
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::{GenomicInterval, Interval, NamedInterval, Strand, StrandedGenomicInterval};

    #[test]
    fn build_stranded_genomic_interval_container() {
        let records = vec![
            StrandedGenomicInterval::new(1, 1, 10, Strand::Forward),
            StrandedGenomicInterval::new(1, 2, 20, Strand::Forward),
            StrandedGenomicInterval::new(1, 3, 30, Strand::Forward),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn build_genomic_interval_container() {
        let records = vec![
            GenomicInterval::new(1, 1, 10),
            GenomicInterval::new(1, 2, 20),
            GenomicInterval::new(1, 3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn build_interval_container() {
        let records = vec![
            Interval::new(1, 10),
            Interval::new(2, 20),
            Interval::new(3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn build_named_interval_container() {
        let records = vec![
            NamedInterval::new("chr1", 1, 10),
            NamedInterval::new("chr1", 2, 20),
            NamedInterval::new("chr1", 3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }
}
