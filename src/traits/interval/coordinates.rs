pub trait Coordinates<T>
where
    T: Copy + Default,
{
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn chr(&self) -> T;
    fn update_start(&mut self, val: &T);
    fn update_end(&mut self, val: &T);
    fn from(other: &Self) -> Self;
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
        fn start(&self) -> usize {
            self.left
        }
        fn end(&self) -> usize {
            self.right
        }
        fn from(other: &Self) -> Self {
            Self {
                left: other.start(),
                right: other.end(),
            }
        }
    }

    // define a custom interval struct for testing
    struct CustomIntervalMeta {
        left: usize,
        right: usize,
        meta: String,
    }
    impl CustomIntervalMeta {
        pub fn meta(&self) -> &str {
            &self.meta
        }

    }
    impl Coordinates<usize> for CustomIntervalMeta {
        fn start(&self) -> usize {
            self.left
        }
        fn end(&self) -> usize {
            self.right
        }
        fn from(other: &Self) -> Self {
            Self {
                left: other.start(),
                right: other.end(),
                meta: other.meta().to_string(),
            }
        }
    }

    #[test]
    fn test_custom_interval() {
        let left = 10;
        let right = 100;
        let a = CustomInterval { left, right };
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 100);
    }

    #[test]
    fn test_custom_interval_with_meta() {
        let left = 10;
        let right = 100;
        let meta = "some_meta".to_string();
        let a = CustomIntervalMeta {
            left,
            right,
            meta,
        };
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 100);
    }
}
