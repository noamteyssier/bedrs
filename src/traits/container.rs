use super::Coordinates;

pub trait Container<T, I: Coordinates<T>> {
    fn records(&self) -> &Vec<I>;
    fn len(&self) -> usize {
        self.records().len()
    }
}

#[cfg(test)]
mod testing {
    use crate::traits::Coordinates;
    use super::Container;

    struct CustomContainer {
        records: Vec<CustomInterval>
    }
    impl Container<usize, CustomInterval> for CustomContainer {
        fn records(&self) -> &Vec<CustomInterval> {
            &self.records
        }
    }

    struct CustomInterval {
        start: usize,
        end: usize,
    }
    impl Coordinates<usize> for CustomInterval {
        fn start(&self) -> &usize {
            &self.start
        }
        fn end(&self) -> &usize {
            &self.end
        }
    }
    
    #[test]
    fn test_custom_container() {
        let records = vec![
            CustomInterval{start: 10, end: 100},
            CustomInterval{start: 10, end: 100},
            CustomInterval{start: 10, end: 100},
            CustomInterval{start: 10, end: 100},
        ];
        let container = CustomContainer { records }; 
        assert_eq!(container.len(), 4);
        assert_eq!(container.records()[0].start(), &10);
        assert_eq!(container.records()[0].end(), &100);
    }
}
