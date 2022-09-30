use crate::traits::{IntervalBounds, ValueBounds};

pub trait Container<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self;
    fn records(&self) -> &Vec<I>;
    fn records_mut(&mut self) -> &mut Vec<I>;
    fn len(&self) -> usize {
        self.records().len()
    }
    fn is_empty(&self) -> bool {
        self.records().is_empty()
    }
    fn sort(&mut self) {
        self.records_mut().sort_unstable();
    }
}

#[cfg(test)]
mod testing {
    use super::Container;
    use crate::{traits::Coordinates, types::Interval};

    struct CustomContainer {
        records: Vec<Interval<usize>>,
    }
    impl Container<usize, Interval<usize>> for CustomContainer {
        fn new(records: Vec<Interval<usize>>) -> Self {
            Self { records }
        }
        fn records(&self) -> &Vec<Interval<usize>> {
            &self.records
        }
        fn records_mut(&mut self) -> &mut Vec<Interval<usize>> {
            &mut self.records
        }
    }

    #[test]
    fn test_custom_container_init() {
        let records = vec![Interval::new(10, 100); 4];
        let container = CustomContainer { records };
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
        let mut container = CustomContainer { records };
        container.sort();
        assert_eq!(container.records()[0].start(), 10);
        assert_eq!(container.records()[1].start(), 15);
        assert_eq!(container.records()[2].start(), 20);
    }

    #[test]
    fn test_custom_container_empty() {
        let records = Vec::new();
        let container = CustomContainer { records };
        assert!(container.is_empty());
    }
}
