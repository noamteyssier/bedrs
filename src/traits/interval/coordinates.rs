use crate::traits::ValueBounds;

/// The main trait representing an interval.
pub trait Coordinates<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn chr(&self) -> T;
    fn update_start(&mut self, val: &T);
    fn update_end(&mut self, val: &T);
    fn update_chr(&mut self, val: &T);
    fn from(other: &Self) -> Self;
    fn update_all_from(&mut self, other: &Self) {
        self.update_chr(&other.chr());
        self.update_endpoints(&other.start(), &other.end());
    }
    fn update_endpoints_from(&mut self, other: &Self) {
        self.update_start(&other.start());
        self.update_end(&other.end());
    }
    fn update_endpoints(&mut self, start: &T, end: &T) {
        self.update_start(start);
        self.update_end(end);
    }
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
        fn chr(&self) -> usize {
            0
        }
        fn update_start(&mut self, val: &usize) {
            self.left = *val;
        }
        fn update_end(&mut self, val: &usize) {
            self.right = *val;
        }
        #[allow(unused)]
        fn update_chr(&mut self, val: &usize) {}
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
        fn chr(&self) -> usize {
            0
        }
        fn update_start(&mut self, val: &usize) {
            self.left = *val;
        }
        fn update_end(&mut self, val: &usize) {
            self.right = *val;
        }
        #[allow(unused)]
        fn update_chr(&mut self, val: &usize) {}
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
        assert_eq!(a.chr(), 0);
    }

    #[test]
    fn test_custom_interval_update() {
        let mut a = CustomInterval {
            left: 10,
            right: 100,
        };
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 100);
        a.update_start(&30);
        a.update_end(&120);
        assert_eq!(a.start(), 30);
        assert_eq!(a.end(), 120);
    }

    #[test]
    fn test_custom_interval_transcode() {
        let a = CustomInterval {
            left: 10,
            right: 100,
        };
        let b = Coordinates::from(&a);
        assert_eq!(a.start(), b.start());
        assert_eq!(a.end(), b.end());
        assert_eq!(a.chr(), b.chr());
    }
    #[test]
    fn test_custom_interval_with_meta() {
        let left = 10;
        let right = 100;
        let meta = "some_meta".to_string();
        let a = CustomIntervalMeta { left, right, meta };
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 100);
        assert_eq!(a.chr(), 0);
    }

    #[test]
    fn test_custom_interval_meta_update() {
        let mut a = CustomIntervalMeta {
            left: 10,
            right: 100,
            meta: String::from("hello"),
        };
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 100);
        a.update_start(&30);
        a.update_end(&120);
        assert_eq!(a.start(), 30);
        assert_eq!(a.end(), 120);
    }

    #[test]
    fn test_custom_interval_meta_transcode() {
        let a = CustomIntervalMeta {
            left: 10,
            right: 100,
            meta: String::from("hello"),
        };
        let b = Coordinates::from(&a);
        assert_eq!(a.start(), b.start());
        assert_eq!(a.end(), b.end());
        assert_eq!(a.chr(), b.chr());
    }
}
