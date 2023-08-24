use crate::{
    traits::{ChromBounds, ValueBounds},
    Intersect, Overlap, Strand, Subtract,
};
use std::cmp::Ordering;

use super::Distance;

/// The main trait representing an interval.
pub trait Coordinates<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn chr(&self) -> &C;

    /// Return the strand of the interval, if it has one.
    ///
    /// This is a default implementation that returns `None` for
    /// intervals that do not have a strand.
    fn strand(&self) -> Option<Strand> {
        None
    }

    fn update_start(&mut self, val: &T);
    fn update_end(&mut self, val: &T);
    fn update_chr(&mut self, val: &C);
    fn from(other: &Self) -> Self;
    fn len(&self) -> T {
        self.end().sub(self.start())
    }
    fn update_all(&mut self, chr: &C, start: &T, end: &T) {
        self.update_chr(chr);
        self.update_endpoints(start, end);
    }
    fn update_endpoints(&mut self, start: &T, end: &T) {
        self.update_start(start);
        self.update_end(end);
    }
    fn update_all_from<I: Coordinates<C, T>>(&mut self, other: &I) {
        self.update_chr(&other.chr());
        self.update_endpoints(&other.start(), &other.end());
    }
    fn update_endpoints_from<I: Coordinates<C, T>>(&mut self, other: &I) {
        self.update_start(&other.start());
        self.update_end(&other.end());
    }
    fn extend_left(&mut self, val: &T) {
        self.update_start(&self.start().sub(*val));
    }
    fn extend_right(&mut self, val: &T) {
        self.update_end(&self.end().add(*val));
    }
    fn extend(&mut self, val: &T) {
        self.extend_left(val);
        self.extend_right(val);
    }
    fn coord_cmp<I: Coordinates<C, T>>(&self, other: &I) -> Ordering {
        match self.chr().cmp(&other.chr()) {
            Ordering::Equal => match self.start().cmp(&other.start()) {
                Ordering::Equal => self.end().cmp(&other.end()),
                order => order,
            },
            order => order,
        }
    }
    /// Compare two intervals, but bias the `other` interval to extend
    /// further to the left by `bias` units.
    ///
    /// Used to find the lower bound of an interval in a sorted container
    /// where the maximum range of the intervals is known a priori.
    fn biased_coord_cmp<I: Coordinates<C, T>>(&self, other: &I, bias: T) -> Ordering {
        match self.chr().cmp(&other.chr()) {
            Ordering::Equal => {
                let comp = if other.start() < bias {
                    None // can't compare the intervals since they both bias below zero
                } else {
                    Some(self.start().cmp(&other.start().sub(bias)))
                };
                if let Some(comp) = comp {
                    match comp {
                        Ordering::Equal => self.end().cmp(&other.end()),
                        order => order,
                    }
                } else {
                    Ordering::Equal
                }
            }
            order => order,
        }
    }
    fn biased_lt<I: Coordinates<C, T>>(&self, other: &I, bias: T) -> bool {
        self.biased_coord_cmp(other, bias) == Ordering::Less
    }
    fn lt<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Less
    }
    fn gt<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Greater
    }
    fn eq<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Equal
    }
}

impl<I, C, T> Distance<C, T> for I
where
    I: Coordinates<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<I, C, T> Intersect<C, T> for I
where
    I: Coordinates<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<I, C, T> Overlap<C, T> for I
where
    I: Coordinates<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

impl<I, C, T> Subtract<C, T> for I
where
    I: Coordinates<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, Interval};

    // define a custom interval struct for testing
    struct CustomInterval {
        left: usize,
        right: usize,
    }
    impl Coordinates<usize, usize> for CustomInterval {
        fn start(&self) -> usize {
            self.left
        }
        fn end(&self) -> usize {
            self.right
        }
        fn chr(&self) -> &usize {
            &0
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
    impl Coordinates<usize, usize> for CustomIntervalMeta {
        fn start(&self) -> usize {
            self.left
        }
        fn end(&self) -> usize {
            self.right
        }
        fn chr(&self) -> &usize {
            &0
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
        assert_eq!(*a.chr(), 0);
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
        assert_eq!(*a.chr(), 0);
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

    #[test]
    fn test_convenience_methods() {
        let a = Interval::new(10, 20);
        let b = Interval::new(30, 50);
        let c = Interval::new(30, 50);
        assert!(a.lt(&b));
        assert!(b.gt(&a));
        assert!(b.eq(&c));
    }

    #[test]
    fn test_extend_left() {
        let mut a = Interval::new(10, 20);
        let val = 5;
        a.extend_left(&val);
        assert_eq!(a.start(), 5);
        assert_eq!(a.end(), 20);
    }

    #[test]
    fn test_extend_right() {
        let mut a = Interval::new(10, 20);
        let val = 5;
        a.extend_right(&val);
        assert_eq!(a.start(), 10);
        assert_eq!(a.end(), 25);
    }

    #[test]
    fn test_extend_both() {
        let mut a = Interval::new(10, 20);
        let val = 5;
        a.extend(&val);
        assert_eq!(a.start(), 5);
        assert_eq!(a.end(), 25);
    }
}
