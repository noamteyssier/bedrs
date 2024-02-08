use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    Coordinates, IntervalIterOwned, IntervalIterRef,
};
use anyhow::{bail, Result};
use num_traits::zero;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn new(records: Vec<I>) -> Self {
        let max_len = records.iter().map(Coordinates::len).max();
        Self {
            records,
            is_sorted: false,
            max_len,
            _phantom_c: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
    #[must_use]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    pub fn records(&self) -> &Vec<I> {
        &self.records
    }
    pub fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.records
    }
    pub fn records_owned(self) -> Vec<I> {
        self.records
    }
    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    pub fn set_unsorted(&mut self) {
        self.is_sorted = false;
    }
    pub fn sorted_mut(&mut self) -> &mut bool {
        &mut self.is_sorted
    }
    pub fn max_len(&self) -> Option<T> {
        self.max_len
    }
    pub fn max_len_mut(&mut self) -> &mut Option<T> {
        &mut self.max_len
    }
    /// Returns the span of the interval set
    pub fn span(&self) -> Result<I> {
        if self.is_empty() {
            bail!("Cannot get span of empty interval set")
        } else if !self.is_sorted() {
            bail!("Cannot get span of unsorted interval set")
        }
        let Some(first) = self.records().first() else {
            bail!("Cannot recover the first interval")
        };
        let Some(last) = self.records().last() else {
            bail!("Cannot recover the last interval")
        };
        if first.chr() != last.chr() {
            bail!("Cannot get span of interval set spanning multiple chromosomes")
        }
        let mut iv = I::empty();
        iv.update_chr(first.chr());
        iv.update_start(&first.start());
        iv.update_end(&last.end());
        Ok(iv)
    }
    pub fn iter(&self) -> IntervalIterRef<I, C, T> {
        IntervalIterRef::new(self.records())
    }
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntervalIterOwned<I, C, T> {
        IntervalIterOwned::new(self.records_owned())
    }

    /// Sets the internal state to sorted
    ///
    /// >> This would likely not be used directly by the user.
    /// >> If you are creating an interval set from presorted
    /// >> intervals use the `from_sorted()` method instead of
    /// >> the `new()` method.
    pub fn set_sorted(&mut self) {
        *self.sorted_mut() = true;
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    pub fn sort(&mut self) {
        self.records_mut().sort_unstable_by(Coordinates::coord_cmp);
        self.set_sorted();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    /// but parallelizes the sorting.
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.records_mut()
            .par_sort_unstable_by(Coordinates::coord_cmp);
        self.set_sorted();
    }

    /// Updates the maximum length of the intervals in the container
    /// if the new interval is longer than the current maximum length.
    pub fn update_max_len<Iv, Co, To>(&mut self, interval: &Iv)
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

    /// Inserts a new interval into the container
    ///
    /// This will not sort the container after insertion.
    /// If you need to sort the container after insertion
    /// use the `insert_sorted()` method instead.
    ///
    /// This is more efficient if you are inserting many
    /// intervals at once.
    pub fn insert(&mut self, interval: I) {
        self.update_max_len(&interval);
        self.records_mut().push(interval);
        self.set_unsorted();
    }

    /// Inserts a new interval into the container and sorts the container
    /// after insertion.
    ///
    /// This is less efficient than the `insert()` method if you are
    /// inserting many intervals at once.
    pub fn insert_sorted(&mut self, interval: I) {
        self.insert(interval);
        self.sort();
    }

    /// Creates a new container from presorted intervals
    ///
    /// First this validates that the intervals are truly presorted.
    pub fn from_sorted(records: Vec<I>) -> Result<Self, SetError> {
        if Self::valid_interval_sorting(&records) {
            Ok(Self::from_sorted_unchecked(records))
        } else {
            Err(SetError::UnsortedIntervals)
        }
    }

    /// Creates a new container from presorted intervals without
    /// validating if the intervals are truly presorted.
    #[must_use]
    pub fn from_sorted_unchecked(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.set_sorted();
        set
    }

    /// Creates a new *sorted* container from unsorted intervals
    #[must_use]
    pub fn from_unsorted(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.sort();
        set
    }

    /// Validates that a set of intervals are sorted
    #[must_use]
    pub fn valid_interval_sorting(records: &[I]) -> bool {
        records
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, rec)| (rec, &records[idx - 1]))
            .all(|(a, b)| a.coord_cmp(b).is_ge())
    }

    /// Applies a mutable function to each interval in the container
    pub fn apply_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut I),
    {
        self.records_mut().iter_mut().for_each(f);
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::{Bed3, Coordinates, Interval, Strand, StrandedBed3};
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    // --------------------- //
    // Base Interval Testing //
    // --------------------- //

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
    fn test_base_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_from_iterator() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_base_serialization() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<Interval<usize>, usize, usize> =
            deserialize(&serialized).unwrap();
        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_base_par_sort() {
        let n_intervals = 10;
        let records = vec![Interval::new(10, 100); n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }

    // ------------------------ //
    // Genomic Interval Testing //
    // ------------------------ //

    #[test]
    fn build_genomic_interval_container() {
        let records = vec![
            Bed3::new(1, 1, 10),
            Bed3::new(1, 2, 20),
            Bed3::new(1, 3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_genomic_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![Bed3::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![Bed3::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_empty_iterator() {
        let records: Vec<Bed3<usize, usize>> = vec![];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), 0);
        assert!(set.max_len().is_none());
        assert!(set.span().is_err());
    }

    #[test]
    fn test_genomic_span() {
        let intervals = vec![Bed3::new(1, 10, 100), Bed3::new(1, 20, 200)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        assert!(set.span().unwrap().eq(&Bed3::new(1, 10, 200)));
    }

    #[test]
    fn test_genomic_span_errors() {
        let intervals = vec![Bed3::new(1, 10, 100), Bed3::new(2, 20, 200)];
        let mut set = IntervalContainer::from_iter(intervals);
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
    fn test_genomic_serialization() {
        let n_intervals = 10;
        let records = vec![Bed3::new(1, 10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<Bed3<usize, usize>, usize, usize> =
            deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(&iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_genomic_par_sort() {
        let n_intervals = 10;
        let records = vec![Bed3::new(1, 10, 100); n_intervals];
        let mut set = IntervalContainer::new(records.clone());
        set.par_sort();
        for (iv1, iv2) in set.records().iter().zip(records.iter()) {
            assert!(iv1.eq(&iv2));
        }
    }

    // ------------------------- //
    // Stranded Interval Testing //
    // ------------------------- //

    #[test]
    fn build_stranded_genomic_interval_container() {
        let records = vec![
            StrandedBed3::new(1, 1, 10, Strand::Forward),
            StrandedBed3::new(1, 2, 20, Strand::Forward),
            StrandedBed3::new(1, 3, 30, Strand::Forward),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_stranded_genomic_init_from_records() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_set_sorted() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Reverse); n_intervals];
        let mut set = IntervalContainer::new(records);
        assert!(!set.is_sorted);
        set.set_sorted();
        assert!(set.is_sorted());
    }

    #[test]
    fn test_stranded_genomic_set_records_mut() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Forward); n_intervals];
        let mut set = IntervalContainer::new(records);

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Forward);
        });

        set.records_mut().iter_mut().for_each(|r| {
            r.set_strand(Strand::Reverse);
        });

        set.records().iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Reverse);
        });
    }

    #[test]
    fn test_span_empty() {
        let set: IntervalContainer<StrandedBed3<u32>, u32, u32> = IntervalContainer::new(vec![]);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_unsorted() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Forward); n_intervals];
        let set = IntervalContainer::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span_multiple_chr() {
        let n_intervals = 10;
        let mut records = vec![StrandedBed3::new(1, 10, 100, Strand::Forward); n_intervals];
        records.push(StrandedBed3::new(2, 10, 100, Strand::Forward));
        let set = IntervalContainer::new(records);
        let span = set.span();
        assert!(span.is_err());
    }

    #[test]
    fn test_span() {
        let records = vec![
            StrandedBed3::new(1, 10, 100, Strand::Forward),
            StrandedBed3::new(1, 1000, 2000, Strand::Forward),
        ];
        let set = IntervalContainer::from_sorted(records).unwrap();
        let span = set.span().unwrap();
        assert_eq!(span.start(), 10);
        assert_eq!(span.end(), 2000);
    }

    #[test]
    fn test_sort() {
        let records = vec![
            StrandedBed3::new(1, 1000, 2000, Strand::Reverse),
            StrandedBed3::new(1, 1000, 2000, Strand::Forward),
            StrandedBed3::new(1, 1000, 2000, Strand::Unknown),
            StrandedBed3::new(1, 10, 100, Strand::Reverse),
            StrandedBed3::new(1, 10, 100, Strand::Forward),
            StrandedBed3::new(1, 10, 100, Strand::Unknown),
            StrandedBed3::new(2, 1000, 2000, Strand::Reverse),
            StrandedBed3::new(2, 1000, 2000, Strand::Forward),
            StrandedBed3::new(2, 1000, 2000, Strand::Unknown),
            StrandedBed3::new(2, 10, 100, Strand::Reverse),
            StrandedBed3::new(2, 10, 100, Strand::Forward),
            StrandedBed3::new(2, 10, 100, Strand::Unknown),
        ];
        let set = IntervalContainer::from_unsorted(records);
        assert!(set.is_sorted());
        let vec = set.records();
        assert!(vec[0].eq(&StrandedBed3::new(1, 10, 100, Strand::Forward)));
        assert!(vec[1].eq(&StrandedBed3::new(1, 10, 100, Strand::Reverse)));
        assert!(vec[2].eq(&StrandedBed3::new(1, 10, 100, Strand::Unknown)));
        assert!(vec[3].eq(&StrandedBed3::new(1, 1000, 2000, Strand::Forward)));
        assert!(vec[4].eq(&StrandedBed3::new(1, 1000, 2000, Strand::Reverse)));
        assert!(vec[5].eq(&StrandedBed3::new(1, 1000, 2000, Strand::Unknown)));
        assert!(vec[6].eq(&StrandedBed3::new(2, 10, 100, Strand::Forward)));
        assert!(vec[7].eq(&StrandedBed3::new(2, 10, 100, Strand::Reverse)));
        assert!(vec[8].eq(&StrandedBed3::new(2, 10, 100, Strand::Unknown)));
        assert!(vec[9].eq(&StrandedBed3::new(2, 1000, 2000, Strand::Forward)));
        assert!(vec[10].eq(&StrandedBed3::new(2, 1000, 2000, Strand::Reverse)));
        assert!(vec[11].eq(&StrandedBed3::new(2, 1000, 2000, Strand::Unknown)));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Reverse); n_intervals];
        let set = IntervalContainer::new(records);
        let serialized = serialize(&set).unwrap();
        let deserialized: IntervalContainer<StrandedBed3<usize>, usize, usize> =
            deserialize(&serialized).unwrap();

        for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
            assert!(iv1.eq(iv2));
        }
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_par_sort() {
        let n_intervals = 10;
        let records = vec![StrandedBed3::new(1, 10, 100, Strand::Reverse); n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }

    #[test]
    fn test_container_init_new() {
        let records = vec![
            Interval::new(15, 25),
            Interval::new(10, 20),
            Interval::new(5, 15),
        ];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), 3);
        assert!(!set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.records()[0].start(), 15);
    }

    #[test]
    fn test_container_init_from_sorted() {
        let records = vec![
            Interval::new(5, 10),
            Interval::new(10, 15),
            Interval::new(15, 20),
        ];
        let set = IntervalContainer::from_sorted(records).unwrap();
        assert_eq!(set.len(), 3);
        assert!(set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.records()[0].start(), 5);
    }

    #[test]
    fn test_container_init_from_unsorted() {
        let records = vec![
            Interval::new(15, 25),
            Interval::new(10, 20),
            Interval::new(5, 15),
        ];
        let set = IntervalContainer::from_unsorted(records);
        assert_eq!(set.len(), 3);
        assert!(set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.records()[0].start(), 5);
    }

    #[test]
    fn test_container_init_from_sorted_false_sorting() {
        let records = vec![
            Interval::new(10, 15),
            Interval::new(5, 10),
            Interval::new(15, 20),
        ];
        let set = IntervalContainer::from_sorted(records);
        assert!(set.is_err());
    }

    #[test]
    fn test_container_apply_mut() {
        let records = vec![
            Interval::new(15, 25),
            Interval::new(10, 20),
            Interval::new(5, 15),
        ];
        let mut set = IntervalContainer::from_unsorted(records);
        set.apply_mut(|rec| rec.extend(&2, None));
        assert_eq!(set.records()[0].start(), 3);
        assert_eq!(set.records()[0].end(), 17);
        assert_eq!(set.records()[1].start(), 8);
        assert_eq!(set.records()[1].end(), 22);
        assert_eq!(set.records()[2].start(), 13);
        assert_eq!(set.records()[2].end(), 27);
    }

    #[test]
    fn test_container_insert() {
        let mut set = IntervalContainer::empty();
        set.insert(Interval::new(15, 25));
        set.insert(Interval::new(10, 20));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_container_insert_sorted() {
        let mut set = IntervalContainer::empty();
        set.insert_sorted(Interval::new(15, 25));
        set.insert_sorted(Interval::new(10, 20));
        assert_eq!(set.len(), 2);
        assert_eq!(set.records()[0].start(), 10);
        assert!(set.is_sorted());
    }

    #[test]
    fn container_iter() {
        let records = vec![
            Interval::new(15, 25),
            Interval::new(10, 20),
            Interval::new(5, 15),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let mut iter = set.iter();
        assert_eq!(iter.next().unwrap().start(), 5);
        assert_eq!(iter.next().unwrap().start(), 10);
        assert_eq!(iter.next().unwrap().start(), 15);
    }
}
