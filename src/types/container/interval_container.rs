use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    IntervalIterOwned, IntervalIterRef,
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
impl<I, C, T> IntervalContainer<I, C, T>
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
    fn set_sorted(&mut self) {
        *self.sorted_mut() = true;
    }
    fn set_unsorted(&mut self) {
        *self.sorted_mut() = false;
    }
    fn len(&self) -> usize {
        self.records.len()
    }
    fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    fn sort(&mut self) {
        self.records_mut().sort_unstable_by(|a, b| a.coord_cmp(b));
        self.set_sorted();
    }
    fn update_max_len<Iv, Co, To>(&mut self, interval: &Iv)
    where
        Iv: IntervalBounds<Co, To>,
        Co: ChromBounds,
        To: ValueBounds + Into<T>,
    {
        if let Some(max_len) = self.max_len() {
            if interval.len().into() > max_len {
                *self.max_len_mut() = Some(interval.len().into());
            }
        } else {
            *self.max_len_mut() = Some(interval.len().into());
        }
    }
    fn insert(&mut self, interval: I) {
        self.update_max_len(&interval);
        self.records_mut().push(interval);
        self.set_unsorted();
    }
    fn insert_sorted(&mut self, interval: I) {
        self.insert(interval);
        self.sort();
    }
    fn records(&self) -> &[I] {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.records
    }
    fn records_owned(self) -> Vec<I> {
        self.records
    }
    fn sorted_mut(&mut self) -> &mut bool {
        &mut self.is_sorted
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
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
    /// Creates a new container from presorted intervals
    ///
    /// First this validates that the intervals are truly presorted.
    fn from_sorted(records: Vec<I>) -> Result<Self, SetError> {
        if Self::valid_interval_sorting(&records) {
            Ok(Self::from_sorted_unchecked(records))
        } else {
            Err(SetError::UnsortedIntervals)
        }
    }

    /// Creates a new container from presorted intervals without
    /// validating if the intervals are truly presorted.
    fn from_sorted_unchecked(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.set_sorted();
        set
    }

    /// Creates a new *sorted* container from unsorted intervals
    fn from_unsorted(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.sort();
        set
    }

    /// Validates that a set of intervals are sorted
    fn valid_interval_sorting(records: &Vec<I>) -> bool {
        records
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, rec)| (rec, &records[idx - 1]))
            .all(|(a, b)| a.coord_cmp(b).is_ge())
    }

    /// Applies a mutable function to each interval in the container
    fn apply_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut I),
    {
        self.records_mut().iter_mut().for_each(f);
    }

    fn iter(&self) -> IntervalIterRef<I, C, T> {
        IntervalIterRef::new(&self.records())
    }

    fn into_iter(self) -> IntervalIterOwned<I, C, T> {
        IntervalIterOwned::new(self.records_owned())
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
