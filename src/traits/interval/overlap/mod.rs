mod stranded;
mod unstranded;

use crate::traits::{ChromBounds, Coordinates, ValueBounds};
pub use stranded::StrandedOverlap;
pub use unstranded::UnstrandedOverlap;

/// A trait to measure overlaps between intervals implementing `Coordinates`
pub trait Overlap<C, T>:
    Coordinates<C, T> + StrandedOverlap<C, T> + UnstrandedOverlap<C, T>
where
    Self: Sized,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Returns true if the two intervals are on the same chromosome.
    ///
    /// ``` text
    /// (Self)   chr1:  |---------|
    /// (Other)  chr2:                |---------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 250);
    /// let interval3 = Bed3::new(2, 100, 200);
    ///
    /// assert!(interval1.bounded_chr(&interval2));
    /// assert!(!interval1.bounded_chr(&interval3));
    /// ```
    fn bounded_chr<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        other.chr() == self.chr()
    }

    /// Returns true if the two intervals are on the same strand.
    ///
    /// ``` text
    /// (Self)     |--------->
    /// (Other)       |--------->
    /// ====================================
    /// true
    ///
    /// (Self)     <---------|
    /// (Other)        <---------|
    /// ====================================
    /// true
    ///
    /// (Self)     |--------->
    /// (Other)        <---------|
    /// ====================================
    /// false
    /// ```
    fn bounded_strand<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        match (self.strand(), other.strand()) {
            (Some(s1), Some(s2)) => s1 == s2,
            _ => true,
        }
    }

    /// Returns true if the two intervals overlap.
    ///
    /// Does not consider the chromosome.
    ///
    /// Measured as bool and of:
    /// * `self.start` is less than `other.end`
    /// * `self.end` is greater than `other.start`
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)     |--------|
    ///
    /// or
    ///
    /// (Self)      |--------|
    /// (Other)   |--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{BaseInterval, Overlap};
    ///
    /// // base interval
    /// let interval1 = BaseInterval::new(100, 200);
    ///
    /// // overlapping on right
    /// let interval2 = BaseInterval::new(150, 250);
    ///
    /// // overlapping on left
    /// let interval3 = BaseInterval::new(50, 150);
    ///
    /// // non-overlapping
    /// let interval4 = BaseInterval::new(250, 350);
    ///
    /// assert!(interval1.interval_overlap(&interval2));
    /// assert!(interval1.interval_overlap(&interval3));
    /// assert!(!interval1.interval_overlap(&interval4));
    /// ```
    fn interval_overlap<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }

    /// Returns true if the current interval contains the other interval.
    ///
    /// Does not consider the chromosome.
    ///
    /// Measured as bool and of:
    /// * `self.start` is less than `other.start`
    /// * `self.end` is greater than `other.end`
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)     |----|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{BaseInterval, Overlap};
    ///
    /// let interval1 = BaseInterval::new(100, 200);
    /// let interval2 = BaseInterval::new(150, 160);
    ///
    /// assert!(interval1.interval_contains(&interval2));
    /// ```
    fn interval_contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    /// Returns true if the current interval borders the other interval.
    ///
    /// Does not consider the chromosome.
    ///
    /// Measured as bool OR of:
    /// * `self.start` is equal to `other.end`
    /// * `self.end` is equal to `other.start`
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)            |--------|
    ///
    /// or
    ///
    /// (Self)             |--------|
    /// (Other)   |--------|
    /// ```
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, Overlap};
    ///
    /// let interval1 = BaseInterval::new(100, 200);
    /// let interval2 = BaseInterval::new(200, 300);
    /// let interval3 = BaseInterval::new(50, 100);
    ///
    /// assert!(interval1.interval_borders(&interval2));
    /// assert!(interval1.interval_borders(&interval3));
    /// ```
    fn interval_borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.start().eq(&other.end()) || self.end().eq(&other.start())
    }

    /// Returns true if the current interval overlaps the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)       |--------|
    ///
    /// or
    ///
    /// (Self)        |--------|
    /// (Other)   |--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 250);
    /// let interval3 = Bed3::new(1, 50, 150);
    /// let interval4 = Bed3::new(2, 150, 250);
    ///
    /// assert!(interval1.overlaps(&interval2));
    /// assert!(interval1.overlaps(&interval3));
    /// assert!(!interval1.overlaps(&interval4));
    /// ```
    fn overlaps<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_overlap(other)
    }

    /// Returns true if the current interval is overlapped by the other
    /// for the requested number of bases - considers both the interval
    /// overlap and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)       |--------|
    /// (n)           |----|
    ///
    ///
    /// or
    ///
    /// (Self)        |--------|
    /// (Other)   |--------|
    /// (n)           |----|
    ///
    /// true if `n` >= `bases`
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 250);
    /// let interval3 = Bed3::new(1, 149, 250);
    /// let interval4 = Bed3::new(1, 151, 250);
    ///
    /// assert!(interval1.overlaps_by(&interval2, 50));
    /// assert!(interval1.overlaps_by(&interval3, 50));
    /// assert!(!interval1.overlaps_by(&interval4, 50));
    /// ```
    fn overlaps_by<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.overlap_size(other).map_or(false, |n| n >= bases)
    }
    /// Returns true if the current interval is overlapped by the other
    /// by the exact number of bases - considers both the interval overlap
    /// and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)       |--------|
    /// (n)           |----|
    ///
    /// or
    ///
    /// (Self)        |--------|
    /// (Other)   |--------|
    /// (n)           |----|
    ///
    /// true if `n` == `bases`
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 250);
    /// let interval3 = Bed3::new(1, 149, 250);
    /// let interval4 = Bed3::new(1, 151, 250);
    ///
    /// assert!(interval1.overlaps_by_exactly(&interval2, 50));
    /// assert!(!interval1.overlaps_by_exactly(&interval3, 50));
    /// assert!(!interval1.overlaps_by_exactly(&interval4, 50));
    /// ```
    fn overlaps_by_exactly<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.overlap_size(other).map_or(false, |n| n == bases)
    }

    /// Returns the number of bases overlapped by the other interval -
    /// considers both the interval overlap and the chromosome.
    /// Returns `None` if the intervals do not overlap.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)       |--------|
    /// (n)           |----|
    ///
    /// or
    ///
    /// (Self)        |--------|
    /// (Other)   |--------|
    /// (n)           |----|
    ///
    /// or
    ///
    /// (Self)    |--------|
    /// (Other)     |----|
    /// (n)         |----|
    ///
    /// or
    ///
    /// (Self)      |----|
    /// (Other)   |--------|
    /// (n)         |----|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 250);
    /// let interval3 = Bed3::new(1, 149, 250);
    /// let interval4 = Bed3::new(1, 151, 250);
    ///
    /// assert_eq!(interval1.overlap_size(&interval2), Some(50));
    /// assert_eq!(interval1.overlap_size(&interval3), Some(51));
    /// assert_eq!(interval1.overlap_size(&interval4), Some(49));
    /// ```
    fn overlap_size<I: Coordinates<C, T>>(&self, other: &I) -> Option<T> {
        if self.overlaps(other) {
            if self.contains(other) {
                Some(other.len())
            } else if other.contains(self) {
                Some(self.len())
            } else if self.start() > other.start() {
                Some(other.end() - self.start())
            } else {
                Some(self.end() - other.start())
            }
        } else {
            None
        }
    }

    /// Returns true if the current interval contains the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)     |----|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 150, 160);
    /// let interval3 = Bed3::new(2, 150, 160);
    ///
    /// assert!(interval1.contains(&interval2));
    /// assert!(!interval1.contains(&interval3));
    /// ```
    fn contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_contains(other)
    }

    /// Returns true if the current interval starts the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)   |-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{Bed3, Overlap};
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 100, 400);
    /// let interval3 = Bed3::new(2, 100, 400);
    /// assert!(interval1.starts(&interval2));
    /// assert!(!interval1.starts(&interval3));
    /// ```
    fn starts<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.start() == other.start() && self.end() < other.end()
    }

    /// Returns true if the current interval ends the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)             |--------|
    /// (Other)   |-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{Bed3, Overlap};
    /// let interval1 = Bed3::new(1, 300, 400);
    /// let interval2 = Bed3::new(1, 100, 400);
    /// let interval3 = Bed3::new(2, 100, 400);
    /// assert!(interval1.ends(&interval2));
    /// assert!(!interval1.ends(&interval3));
    /// ```
    fn ends<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.start() > other.start() && self.end() == other.end()
    }

    /// Returns true if the current interval equals the other -
    /// considers both the interval overlap and the chromosome.
    /// ```text
    /// (Self)    |--------|
    /// (Other)   |--------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{Bed3, Overlap};
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 100, 200);
    /// let interval3 = Bed3::new(2, 100, 200);
    /// assert!(interval1.equals(&interval2));
    /// assert!(!interval1.equals(&interval3));
    /// ```
    fn equals<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.start() == other.start() && self.end() == other.end()
    }

    /// Returns true if the current interval is during the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)      |----|
    /// (Other)   |----------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{Bed3, Overlap};
    /// let interval1 = Bed3::new(1, 150, 160);
    /// let interval2 = Bed3::new(1, 100, 200);
    /// let interval3 = Bed3::new(2, 100, 200);
    /// let interval4 = Bed3::new(2, 100, 160);
    /// let interval5 = Bed3::new(2, 150, 160);
    /// assert!(interval1.during(&interval2));
    /// assert!(!interval1.during(&interval3));
    /// assert!(!interval1.during(&interval4));
    /// assert!(!interval1.during(&interval5));
    /// ```
    fn during<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.start() > other.start() && self.end() < other.end()
    }

    /// Returns true if the current interval is contained by the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)      |----|
    /// (Other)   |--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 150, 16);
    /// let interval2 = Bed3::new(1, 100, 200);
    /// let interval3 = Bed3::new(2, 100, 200);
    ///
    /// assert!(interval1.contained_by(&interval2));
    /// assert!(!interval1.contained_by(&interval3));
    /// ```
    fn contained_by<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        other.contains(self)
    }

    /// Returns true if the current interval borders the other -
    /// considers both the interval overlap and the chromosome.
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)            |--------|
    ///
    /// or
    ///
    /// (Self)             |--------|
    /// (Other)   |--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{Bed3, Overlap};
    ///
    /// let interval1 = Bed3::new(1, 100, 200);
    /// let interval2 = Bed3::new(1, 200, 300);
    /// let interval3 = Bed3::new(2, 200, 300);
    ///
    /// assert!(interval1.borders(&interval2));
    /// assert!(!interval1.borders(&interval3));
    /// ```
    fn borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_borders(other)
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod testing {
    use super::*;
    use crate::{
        types::{record::Bed3, BaseInterval},
        Coordinates, Strand, StrandedBed3,
    };

    #[test]
    fn test_overlap_self() {
        let a = BaseInterval::new(10, 20);
        assert!(a.overlaps(&a));
    }

    #[test]
    fn test_overlap_reciprocity() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert!(a.overlaps(&b));

        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_overlap_negative_reciprocity() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(25, 35);
        assert!(!a.overlaps(&b));

        let a = BaseInterval::new(25, 35);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_overlap_boundary() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(20, 30);
        assert!(!a.overlaps(&b));
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_self() {
        let a = Bed3::new(1, 10, 20);
        assert!(a.overlaps(&a));
    }

    #[test]
    fn test_genomic_overlap_reciprocity() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(1, 15, 25);
        assert!(a.overlaps(&b));

        let a = Bed3::new(1, 15, 25);
        let b = Bed3::new(1, 10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_negative_reciprocity() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(1, 25, 35);
        assert!(!a.overlaps(&b));

        let a = Bed3::new(1, 25, 35);
        let b = Bed3::new(1, 10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_wrong_chr() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(2, 10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_base_contained() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(15, 25);
        let c = BaseInterval::new(10, 30);
        let d = BaseInterval::new(9, 31);
        assert!(a.contains(&b));
        assert!(b.contained_by(&a));
        assert!(a.contains(&c));
        assert!(a.contained_by(&c));
        assert!(!a.contains(&d));
        assert!(a.contained_by(&d));
    }

    #[test]
    fn test_overlapping_contains() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(15, 25);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_genomic_contained() {
        let a = Bed3::new(1, 10, 30);
        let b = Bed3::new(1, 15, 25);
        let c = Bed3::new(1, 10, 30);
        let d = Bed3::new(1, 9, 31);
        let e = Bed3::new(2, 15, 25);
        assert!(a.contains(&b));
        assert!(b.contained_by(&a));
        assert!(a.contains(&c));
        assert!(a.contained_by(&c));
        assert!(!a.contains(&d));
        assert!(a.contained_by(&d));
        assert!(!a.contains(&e));
        assert!(!e.contained_by(&a));
    }

    #[test]
    fn test_overlap_identity() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn base_borders() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(20, 30);
        assert!(a.borders(&b));
        assert!(b.borders(&a));
    }

    #[test]
    fn genomic_borders() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(1, 20, 30);
        let c = Bed3::new(2, 20, 30);
        assert!(a.borders(&b));
        assert!(b.borders(&a));
        assert!(!a.borders(&c));
        assert!(!c.borders(&a));
    }

    #[test]
    fn overlap_size_lt() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert_eq!(a.overlap_size(&b), Some(5));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(14, 25);
        assert_eq!(a.overlap_size(&b), Some(6));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(16, 25);
        assert_eq!(a.overlap_size(&b), Some(4));
    }

    #[test]
    fn overlap_size_gt() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(5));

        let a = BaseInterval::new(14, 25);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(6));

        let a = BaseInterval::new(16, 25);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(4));
    }

    #[test]
    fn overlap_size_none() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(21, 25);
        assert_eq!(a.overlap_size(&b), None);

        let a = BaseInterval::new(21, 25);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.overlap_size(&b), None);
    }

    #[test]
    fn overlaps_by_lt() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert!(a.overlaps_by(&b, 5));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(16, 25);
        assert!(!a.overlaps_by(&b, 5));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(14, 25);
        assert!(a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_by_gt() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(10, 20);
        assert!(a.overlaps_by(&b, 5));

        let a = BaseInterval::new(16, 25);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps_by(&b, 5));

        let a = BaseInterval::new(14, 25);
        let b = BaseInterval::new(10, 20);
        assert!(a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_by_none() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(21, 25);
        assert!(!a.overlaps_by(&b, 5));

        let a = BaseInterval::new(21, 25);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_exact_lt() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert!(a.overlaps_by_exactly(&b, 5));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(16, 25);
        assert!(!a.overlaps_by_exactly(&b, 5));

        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(14, 25);
        assert!(!a.overlaps_by_exactly(&b, 5));
    }

    #[test]
    fn overlaps_exact_gt() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(10, 20);
        assert!(a.overlaps_by_exactly(&b, 5));

        let a = BaseInterval::new(16, 25);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps_by_exactly(&b, 5));

        let a = BaseInterval::new(14, 25);
        let b = BaseInterval::new(10, 20);
        assert!(!a.overlaps_by_exactly(&b, 5));
    }

    #[test]
    fn overlap_size_contains() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(17, 23);
        assert_eq!(a.overlap_size(&b), Some(b.len()));
    }

    #[test]
    fn overlap_size_contained_by() {
        let a = BaseInterval::new(17, 23);
        let b = BaseInterval::new(15, 25);
        assert_eq!(a.overlap_size(&b), Some(a.len()));
    }

    #[test]
    fn starts() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(10, 30);
        let c = BaseInterval::new(10, 40);
        assert!(a.starts(&b));
        assert!(a.starts(&c));
    }

    #[test]
    fn starts_genomic() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(1, 10, 40);
        let c = Bed3::new(2, 10, 40);
        assert!(a.starts(&b));
        assert!(!a.starts(&c));
    }
    #[test]
    fn ends() {
        let a = BaseInterval::new(30, 40);
        let b = BaseInterval::new(10, 40);
        let c = BaseInterval::new(10, 40);
        assert!(a.ends(&b));
        assert!(a.ends(&c));
    }

    #[test]
    fn ends_genomic() {
        let a = Bed3::new(1, 30, 40);
        let b = Bed3::new(1, 10, 40);
        let c = Bed3::new(2, 10, 40);
        assert!(a.ends(&b));
        assert!(!a.ends(&c));
    }

    #[test]
    fn equals() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(10, 20);
        let c = BaseInterval::new(10, 21);
        assert!(a.equals(&b));
        assert!(!a.equals(&c));
    }

    #[test]
    fn during() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(10, 30);
        let c = BaseInterval::new(15, 30);
        let d = BaseInterval::new(10, 25);
        let e = BaseInterval::new(15, 25);
        assert!(a.during(&b));
        assert!(!a.during(&c));
        assert!(!a.during(&d));
        assert!(!a.during(&e));
    }

    #[test]
    fn bounded_strand_missing_info() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(1, 15, 25);
        let c = StrandedBed3::new(1, 15, 25, Strand::Forward);
        assert!(a.bounded_strand(&b));
        assert!(a.bounded_strand(&c));
        assert!(c.bounded_strand(&a));
    }
}
