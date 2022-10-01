use crate::{
    traits::{IntervalBounds, ValueBounds},
    Bound, Find, Merge,
};

/// The main trait representing a container of intervals.
///
/// Each of the intervals: `I` must impl the `Coordinates` trait.
pub trait Container<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self;
    fn records(&self) -> &Vec<I>;
    fn records_mut(&mut self) -> &mut Vec<I>;
    fn is_sorted(&self) -> bool;
    fn set_sorted(&mut self);
    fn len(&self) -> usize {
        self.records().len()
    }
    fn is_empty(&self) -> bool {
        self.records().is_empty()
    }
    fn sort(&mut self) {
        self.records_mut().sort_unstable_by(|a, b| a.coord_cmp(b));
        self.set_sorted();
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
    use crate::{traits::Coordinates, types::Interval};

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
}
