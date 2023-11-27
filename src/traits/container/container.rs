use super::{closest::Closest, Complement};
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    types::{IntervalIterOwned, IntervalIterRef},
    Bound, Find, Internal, Merge, Sample, SetSubtract,
};
use anyhow::Result;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// The main trait representing a container of intervals.
///
/// Each of the intervals: `I` must impl the `Coordinates` trait.
pub trait Container<C, T, I>
where
    Self: Sized,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Creates a new container from intervals (assumed unsorted)
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{GenomicInterval, Container, IntervalContainer};
    ///
    /// let ivs = vec![
    ///     GenomicInterval::new(1, 1, 10),
    ///     GenomicInterval::new(1, 20, 30),
    ///     GenomicInterval::new(1, 40, 50),
    /// ];
    /// let set = IntervalContainer::new(ivs);
    /// assert_eq!(set.len(), 3);
    /// ```
    fn new(records: Vec<I>) -> Self;

    /// Creates a new empty container
    ///
    /// # Examples
    /// use bedrs::{GenomicInterval, Container, IntervalContainer};
    ///
    /// let set = IntervalContainer::<GenomicInterval<u32>, u32, u32>::empty();
    /// assert_eq!(set.len(), 0);
    /// ```
    fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Returns a reference to the internal interval vector
    ///
    /// # Examples
    /// ```
    /// use bedrs::{GenomicInterval, Container, IntervalContainer};
    ///
    /// let ivs = vec![
    ///     GenomicInterval::new(1, 1, 10),
    ///     GenomicInterval::new(1, 20, 30),
    ///     GenomicInterval::new(1, 40, 50),
    /// ];
    /// let set = IntervalContainer::new(ivs);
    /// assert_eq!(set.records().len(), 3);
    /// ```
    fn records(&self) -> &Vec<I>;

    /// Returns a mutable reference to the internal interval vector
    fn records_mut(&mut self) -> &mut Vec<I>;

    /// Returns the internal records as a vector
    fn records_owned(self) -> Vec<I>;

    /// Returns `true` if the internal vector is sorted
    fn is_sorted(&self) -> bool;

    /// Returns a mutable reference to the internal sorted flag
    fn sorted_mut(&mut self) -> &mut bool;

    /// Returns the maximum length of the intervals in the container
    fn max_len(&self) -> Option<T>;

    /// Returns a mutable reference to the maximum length of the intervals
    /// in the container
    fn max_len_mut(&mut self) -> &mut Option<T>;

    /// Sets the internal state to sorted
    ///
    /// >> This would likely not be used directly by the user.
    /// >> If you are creating an interval set from presorted
    /// >> intervals use the `from_sorted()` method instead of
    /// >> the `new()` method.
    fn set_sorted(&mut self) {
        *self.sorted_mut() = true;
    }

    /// Sets the internal state to unsorted
    fn set_unsorted(&mut self) {
        *self.sorted_mut() = false;
    }

    /// Returns the number of records in the container
    fn len(&self) -> usize {
        self.records().len()
    }

    /// Returns `true` if the container has no intervals
    fn is_empty(&self) -> bool {
        self.records().is_empty()
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    fn sort(&mut self) {
        self.records_mut().sort_unstable_by(|a, b| a.coord_cmp(b));
        self.set_sorted();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    /// but parallelizes the sorting.
    #[cfg(feature = "rayon")]
    fn par_sort(&mut self) {
        self.records_mut()
            .par_sort_unstable_by(|a, b| a.coord_cmp(b));
        self.set_sorted();
    }

    /// Updates the maximum length of the intervals in the container
    /// if the new interval is longer than the current maximum length.
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

    /// Inserts a new interval into the container
    ///
    /// This will not sort the container after insertion.
    /// If you need to sort the container after insertion
    /// use the `insert_sorted()` method instead.
    ///
    /// This is more efficient if you are inserting many
    /// intervals at once.
    fn insert(&mut self, interval: I) {
        self.update_max_len(&interval);
        self.records_mut().push(interval);
        self.set_unsorted();
    }

    /// Inserts a new interval into the container and sorts the container
    /// after insertion.
    ///
    /// This is less efficient than the `insert()` method if you are
    /// inserting many intervals at once.
    fn insert_sorted(&mut self, interval: I) {
        self.insert(interval);
        self.sort();
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

    /// Calculates the span of the container and creates a new interval
    /// representing the span. This must be implemented by the user for
    /// custom containers.
    ///
    /// This is not implemented generically because the span of a container
    /// is not always well defined with container-specific contexts
    /// (think multiple chromosomes or associated metadata to interval sets).
    ///
    /// However one can imagine it as the following:
    ///
    /// ``` text
    /// IV1 : |-----|
    /// IV2 :           |-----|
    /// IV3 :                    |-----|
    /// Span: |------------------------|
    /// ```
    fn span(&self) -> Result<I> {
        unimplemented!("Span is not implemented generically - you will need to implement it for your custom container")
    }

    fn iter(&self) -> IntervalIterRef<I, C, T> {
        IntervalIterRef::new(&self.records())
    }

    fn into_iter(self) -> IntervalIterOwned<I, C, T> {
        IntervalIterOwned::new(self.records_owned())
    }
}

impl<Co, C, T, I> Internal<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> Merge<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> Find<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type ContainerType = Co;
}

impl<Co, C, T, I> Bound<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> Sample<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> SetSubtract<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> Complement<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<Co, C, T, I> Closest<C, T, I> for Co
where
    Co: Container<C, T, I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

#[cfg(test)]
mod testing {
    use super::Container;
    use crate::{traits::Coordinates, types::Interval, IntervalContainer};

    struct CustomContainer {
        records: Vec<Interval<usize>>,
        max_len: Option<usize>,
        is_sorted: bool,
    }
    impl Container<usize, usize, Interval<usize>> for CustomContainer {
        fn new(records: Vec<Interval<usize>>) -> Self {
            let max_len = records.iter().map(|iv| iv.len()).max();
            Self {
                records,
                max_len,
                is_sorted: false,
            }
        }
        fn records(&self) -> &Vec<Interval<usize>> {
            &self.records
        }
        fn records_mut(&mut self) -> &mut Vec<Interval<usize>> {
            &mut self.records
        }
        fn records_owned(self) -> Vec<Interval<usize>> {
            self.records
        }
        fn is_sorted(&self) -> bool {
            self.is_sorted
        }
        fn sorted_mut(&mut self) -> &mut bool {
            &mut self.is_sorted
        }
        fn max_len(&self) -> Option<usize> {
            self.max_len
        }
        fn max_len_mut(&mut self) -> &mut Option<usize> {
            &mut self.max_len
        }
    }

    #[test]
    fn test_custom_container_init() {
        let records = vec![Interval::new(10, 100); 4];
        let container = CustomContainer {
            records,
            max_len: Some(90),
            is_sorted: false,
        };
        assert_eq!(container.len(), 4);
        assert_eq!(container.records()[0].start(), 10);
        assert_eq!(container.records()[0].end(), 100);
    }

    #[test]
    fn test_custom_container_sort() {
        let records = vec![
            Interval::new(20, 30), // 3
            Interval::new(10, 20), // 1
            Interval::new(15, 25), // 2
        ];
        let mut container = CustomContainer {
            records,
            max_len: Some(10),
            is_sorted: false,
        };
        container.sort();
        assert_eq!(container.records()[0].start(), 10);
        assert_eq!(container.records()[1].start(), 15);
        assert_eq!(container.records()[2].start(), 20);
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_custom_container_par_sort() {
        let records = vec![
            Interval::new(20, 30), // 3
            Interval::new(10, 20), // 1
            Interval::new(15, 25), // 2
        ];
        let mut container = CustomContainer {
            records,
            max_len: Some(10),
            is_sorted: false,
        };
        container.par_sort();
        assert_eq!(container.records()[0].start(), 10);
        assert_eq!(container.records()[1].start(), 15);
        assert_eq!(container.records()[2].start(), 20);
    }

    #[test]
    fn test_custom_container_empty() {
        let records = Vec::new();
        let container = CustomContainer {
            records,
            max_len: None,
            is_sorted: false,
        };
        assert!(container.is_empty());
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
        set.apply_mut(|rec| rec.extend(&2));
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
