use anyhow::bail;

use crate::{
    traits::{IntervalBounds, ValueBounds},
    Bound, Find, Merge,
};

/// The main trait representing a container of intervals.
///
/// Each of the intervals: `I` must impl the `Coordinates` trait.
pub trait Container<T, I>
where
    Self: Sized,
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    /// Creates a new container from intervals (assumed unsorted)
    fn new(records: Vec<I>) -> Self;

    /// Returns a reference to the internal interval vector
    fn records(&self) -> &Vec<I>;

    /// Returns a mutable reference to the internal interval vector
    fn records_mut(&mut self) -> &mut Vec<I>;

    /// Returns `true` if the internal vector is sorted
    fn is_sorted(&self) -> bool;

    /// Sets the internal state to sorted
    ///
    /// >> This would likely not be used directly by the user.
    /// >> If you are creating an interval set from presorted
    /// >> intervals use the `from_sorted()` method instead of
    /// >> the `new()` method.
    fn set_sorted(&mut self);

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

    /// Creates a new container from presorted intervals
    ///
    /// First this validates that the intervals are truly presorted.
    fn from_sorted(records: Vec<I>) -> anyhow::Result<Self> {
        if Self::valid_interval_sorting(&records) {
            Ok(Self::from_sorted_unchecked(records))
        } else {
            bail!("Intervals are unsorted!")
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
}

impl<C, T, I> Merge<T, I> for C
where
    C: Container<T, I>,
    I: IntervalBounds<T>,
    T: ValueBounds,
{
}

impl<C, T, I> Find<T, I> for C
where
    C: Container<T, I>,
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    type ContainerType = C;
}

impl<C, T, I> Bound<T, I> for C
where
    C: Container<T, I>,
    I: IntervalBounds<T>,
    T: ValueBounds,
{
}

#[cfg(test)]
mod testing {
    use super::Container;
    use crate::{traits::Coordinates, types::Interval, IntervalSet};

    struct CustomContainer {
        records: Vec<Interval<usize>>,
        is_sorted: bool,
    }
    impl Container<usize, Interval<usize>> for CustomContainer {
        fn new(records: Vec<Interval<usize>>) -> Self {
            Self {
                records,
                is_sorted: false,
            }
        }
        fn records(&self) -> &Vec<Interval<usize>> {
            &self.records
        }
        fn records_mut(&mut self) -> &mut Vec<Interval<usize>> {
            &mut self.records
        }
        fn is_sorted(&self) -> bool {
            self.is_sorted
        }
        fn set_sorted(&mut self) {
            self.is_sorted = true;
        }
    }

    #[test]
    fn test_custom_container_init() {
        let records = vec![Interval::new(10, 100); 4];
        let container = CustomContainer {
            records,
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
            is_sorted: false,
        };
        container.sort();
        assert_eq!(container.records()[0].start(), 10);
        assert_eq!(container.records()[1].start(), 15);
        assert_eq!(container.records()[2].start(), 20);
    }

    #[test]
    fn test_custom_container_empty() {
        let records = Vec::new();
        let container = CustomContainer {
            records,
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
        let set = IntervalSet::new(records);
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
        let set = IntervalSet::from_sorted(records).unwrap();
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
        let set = IntervalSet::from_unsorted(records);
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
        let set = IntervalSet::from_sorted(records);
        assert!(set.is_err());
    }
}
