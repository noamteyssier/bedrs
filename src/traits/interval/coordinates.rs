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
    /// Returns the start coordinate of the interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.start(), 10);
    /// ```
    fn start(&self) -> T;

    /// Returns the end coordinate of the interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.end(), 20);
    /// ```
    fn end(&self) -> T;

    /// Returns a reference to the chromosome of the interval.
    ///
    /// *Note*: A reference is returned in the case that the chromosome
    /// is a large type, such as a `String` or `Vec<u8>`.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.chr(), &1);
    /// ```
    fn chr(&self) -> &C;

    /// Return the strand of the interval, if it has one.
    ///
    /// This is a default implementation that returns `None` for
    /// intervals that do not have a strand.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval, Strand, StrandedGenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.strand(), None);
    ///
    /// let siv = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
    /// assert_eq!(siv.strand(), Some(Strand::Forward));
    /// ```
    fn strand(&self) -> Option<Strand> {
        None
    }

    /// Update the start coordinate of the interval.
    ///
    ///     
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.start(), 10);
    ///
    /// iv.update_start(&5);
    /// assert_eq!(iv.start(), 5);
    /// ```
    fn update_start(&mut self, val: &T);

    /// Update the end coordinate of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.end(), 20);
    ///
    /// iv.update_end(&30);
    /// assert_eq!(iv.end(), 30);
    /// ```
    fn update_end(&mut self, val: &T);

    /// Update the chromosome of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.chr(), &1);
    ///
    /// iv.update_chr(&2);
    /// assert_eq!(iv.chr(), &2);
    /// ```
    fn update_chr(&mut self, val: &C);

    /// Create a new interval with the same coordinates as the current one.
    ///
    /// *Note*: This is less verbose when working with generic types.
    /// In most cases it can be better to use the `copy` or `clone` methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// let new_iv = <GenomicInterval<usize> as Coordinates<usize, usize>>::from(&iv);
    ///
    /// assert!(iv.eq(&new_iv));
    /// ```
    fn from<Iv: Coordinates<C, T>>(other: &Iv) -> Self;

    /// Creates an empty interval.
    fn empty() -> Self;

    /// Calculates the length of the interval across its start and end coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let iv = GenomicInterval::new(1, 10, 20);
    /// assert_eq!(iv.len(), 10);
    /// ```
    fn len(&self) -> T {
        self.end().sub(self.start())
    }

    /// Update all attributes of the interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.update_all(&2, &5, &10);
    /// assert!(iv.eq(&GenomicInterval::new(2, 5, 10)));
    /// ```
    fn update_all(&mut self, chr: &C, start: &T, end: &T) {
        self.update_chr(chr);
        self.update_endpoints(start, end);
    }

    /// Update the endpoints of the interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.update_endpoints(&5, &10);
    /// assert!(iv.eq(&GenomicInterval::new(1, 5, 10)));
    /// ```
    fn update_endpoints(&mut self, start: &T, end: &T) {
        self.update_start(start);
        self.update_end(end);
    }

    /// Update all the attributes of the interval from another interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.update_all_from(&GenomicInterval::new(2, 5, 10));
    /// assert!(iv.eq(&GenomicInterval::new(2, 5, 10)));
    /// ```
    fn update_all_from<I: Coordinates<C, T>>(&mut self, other: &I) {
        self.update_chr(&other.chr());
        self.update_endpoints(&other.start(), &other.end());
    }

    /// Update only the endpoints of the interval from another interval.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.update_endpoints_from(&GenomicInterval::new(2, 5, 10));
    /// assert!(iv.eq(&GenomicInterval::new(1, 5, 10)));
    /// ```
    fn update_endpoints_from<I: Coordinates<C, T>>(&mut self, other: &I) {
        self.update_start(&other.start());
        self.update_end(&other.end());
    }

    /// Extend the interval to the left by a value.
    /// This is equivalent to subtracting the value from the start coordinate.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.extend_left(&5);
    /// assert!(iv.eq(&GenomicInterval::new(1, 5, 20)));
    /// ```
    fn extend_left(&mut self, val: &T) {
        self.update_start(&self.start().sub(*val));
    }

    /// Extend the interval to the right by a value.
    /// This is equivalent to adding the value to the end coordinate.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.extend_right(&5);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 25)));
    /// ```
    fn extend_right(&mut self, val: &T) {
        self.update_end(&self.end().add(*val));
    }

    /// Extend the interval to the left and right by a value.
    /// This is equivalent to subtracting the value from the start coordinate
    /// and adding the value to the end coordinate.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let mut iv = GenomicInterval::new(1, 10, 20);
    /// assert!(iv.eq(&GenomicInterval::new(1, 10, 20)));
    ///
    /// iv.extend(&5);
    /// assert!(iv.eq(&GenomicInterval::new(1, 5, 25)));
    /// ```
    fn extend(&mut self, val: &T) {
        self.extend_left(val);
        self.extend_right(val);
    }

    /// Compare two intervals by their genomic coordinates.
    ///
    /// # Examples
    /// ```
    /// use bedrs::{Coordinates, GenomicInterval};
    ///
    /// let a = GenomicInterval::new(1, 10, 20);
    /// let b = GenomicInterval::new(1, 10, 20);
    /// let c = GenomicInterval::new(1, 20, 30);
    /// let d = GenomicInterval::new(2, 10, 20);
    /// let e = GenomicInterval::new(1, 5, 10);
    ///
    /// // a == b
    /// assert_eq!(a.coord_cmp(&b), std::cmp::Ordering::Equal);
    ///
    /// // a < c
    /// assert_eq!(a.coord_cmp(&c), std::cmp::Ordering::Less);
    ///
    /// // a < d
    /// assert_eq!(a.coord_cmp(&d), std::cmp::Ordering::Less);
    ///
    /// // a > e
    /// assert_eq!(a.coord_cmp(&e), std::cmp::Ordering::Greater);
    /// ```
    fn coord_cmp<I: Coordinates<C, T>>(&self, other: &I) -> Ordering {
        match self.chr().cmp(&other.chr()) {
            Ordering::Equal => match self.start().cmp(&other.start()) {
                Ordering::Equal => match self.end().cmp(&other.end()) {
                    Ordering::Equal => self.strand().cmp(&other.strand()),
                    order => order,
                },
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
                        Ordering::Equal => match self.end().cmp(&other.end()) {
                            Ordering::Equal => self.strand().cmp(&other.strand()),
                            order => order,
                        },
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
        fn empty() -> Self {
            Self { left: 0, right: 0 }
        }
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
        fn from<Iv: Coordinates<usize, usize>>(other: &Iv) -> Self {
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
    impl Coordinates<usize, usize> for CustomIntervalMeta {
        fn empty() -> Self {
            Self {
                left: 0,
                right: 0,
                meta: String::new(),
            }
        }
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
        fn from<Iv: Coordinates<usize, usize>>(other: &Iv) -> Self {
            Self {
                left: other.start(),
                right: other.end(),
                meta: String::new(),
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
        let b: CustomInterval = Coordinates::from(&a);
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
        let b: CustomIntervalMeta = Coordinates::from(&a);
        assert_eq!(a.start(), b.start());
        assert_eq!(a.end(), b.end());
        assert_eq!(a.chr(), b.chr());
        assert_ne!(a.meta, b.meta);
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
