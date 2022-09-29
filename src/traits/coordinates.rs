pub trait Coordinates<T> {
    fn start(&self) -> &T;
    fn end(&self) -> &T;
}

#[cfg(test)]
mod testing {
    use crate::traits::Coordinates;

    // define a custom interval struct for testing
    struct CustomInterval {
        left: usize,
        right: usize,
    }
    impl Coordinates<usize> for CustomInterval {
        fn start(&self) -> &usize {
            &self.left
        }
        fn end(&self) -> &usize {
            &self.right
        }
    }
    
    // define a custom interval struct for testing
    struct CustomIntervalMeta<'a> {
        left: usize,
        right: usize,
        _meta: &'a str,
    }
    impl<'a> Coordinates<usize> for CustomIntervalMeta<'a> {
        fn start(&self) -> &usize {
            &self.left
        }
        fn end(&self) -> &usize {
            &self.right
        }
    }
    
    #[test]
    fn test_custom_interval() {
        let left = 10;
        let right = 100;
        let a = CustomInterval{left, right};
        assert_eq!(a.start(), &10);
        assert_eq!(a.end(), &100);
    }

    #[test]
    fn test_custom_interval_with_meta() {
        let left = 10;
        let right = 100;
        let meta = "some_meta";
        let a = CustomIntervalMeta{left, right, _meta: meta};
        assert_eq!(a.start(), &10);
        assert_eq!(a.end(), &100);
    }

}
