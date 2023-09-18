use super::Coordinates;
use crate::traits::{ChromBounds, ValueBounds};

/// A trait to measure overlaps between intervals implementing `Coordinates`
pub trait Overlap<C, T>: Coordinates<C, T>
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 250);
    /// let interval3 = GenomicInterval::new(2, 100, 200);
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
    /// use bedrs::{Interval, Overlap};
    ///
    /// // base interval
    /// let interval1 = Interval::new(100, 200);
    ///
    /// // overlapping on right
    /// let interval2 = Interval::new(150, 250);
    ///
    /// // overlapping on left
    /// let interval3 = Interval::new(50, 150);
    ///
    /// // non-overlapping
    /// let interval4 = Interval::new(250, 350);
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
    /// use bedrs::{Interval, Overlap};
    ///
    /// let interval1 = Interval::new(100, 200);
    /// let interval2 = Interval::new(150, 160);
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
    /// use bedrs::{Interval, Overlap};
    ///
    /// let interval1 = Interval::new(100, 200);
    /// let interval2 = Interval::new(200, 300);
    /// let interval3 = Interval::new(50, 100);
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 250);
    /// let interval3 = GenomicInterval::new(1, 50, 150);
    /// let interval4 = GenomicInterval::new(2, 150, 250);
    ///
    /// assert!(interval1.overlaps(&interval2));
    /// assert!(interval1.overlaps(&interval3));
    /// assert!(!interval1.overlaps(&interval4));
    /// ```
    fn overlaps<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_overlap(other)
    }

    /// Returns true if the current interval overlaps the other
    /// and both intervals are on the same chromosome and strand.
    ///
    /// Considers all three of:
    /// 1. The chromosome
    /// 2. The strand
    /// 3. The interval overlap
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)       |-------->
    ///
    /// or
    ///
    /// (Self)        <--------|
    /// (Other)   <--------|
    /// ```
    fn stranded_overlaps<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.bounded_strand(other) && self.interval_overlap(other)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 250);
    /// let interval3 = GenomicInterval::new(1, 149, 250);
    /// let interval4 = GenomicInterval::new(1, 151, 250);
    ///
    /// assert!(interval1.overlaps_by(&interval2, 50));
    /// assert!(interval1.overlaps_by(&interval3, 50));
    /// assert!(!interval1.overlaps_by(&interval4, 50));
    /// ```
    fn overlaps_by<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.overlap_size(other).map_or(false, |n| n >= bases)
    }

    fn stranded_overlaps_by<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.stranded_overlap_size(other)
            .map_or(false, |n| n >= bases)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 250);
    /// let interval3 = GenomicInterval::new(1, 149, 250);
    /// let interval4 = GenomicInterval::new(1, 151, 250);
    ///
    /// assert!(interval1.overlaps_by_exactly(&interval2, 50));
    /// assert!(!interval1.overlaps_by_exactly(&interval3, 50));
    /// assert!(!interval1.overlaps_by_exactly(&interval4, 50));
    /// ```
    fn overlaps_by_exactly<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.overlap_size(other).map_or(false, |n| n == bases)
    }

    fn stranded_overlaps_by_exactly<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.stranded_overlap_size(other)
            .map_or(false, |n| n == bases)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 250);
    /// let interval3 = GenomicInterval::new(1, 149, 250);
    /// let interval4 = GenomicInterval::new(1, 151, 250);
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

    /// Returns the number of bases overlapped by the other interval if
    /// those intervals are on the same strand
    ///
    /// Considers all three cases:
    /// 1. Both intervals are on the same strand
    /// 2. Both intervals are on the same chromosome
    /// 3. Both intervals overlap
    ///
    /// Returns `None` if the intervals do not overlap or are not on the same strand
    /// or chromosome.
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)       |-------->
    /// (n)           |---->
    ///
    /// or
    /// (Self)    <--------|
    /// (Other)       <--------|
    /// (n)           <----|
    ///
    /// or
    /// (Self)    <--------|
    /// (Other)       |-------->
    ///               None
    ///
    /// or
    /// (Self)    |-------->
    /// (Other)       <--------|
    ///               None
    /// ```
    ///
    /// # Example
    /// ```
    /// use bedrs::{StrandedGenomicInterval, Strand, Coordinates, Overlap};
    ///
    /// let a = StrandedGenomicInterval::new(1, 100, 200, Strand::Forward);
    /// let b = StrandedGenomicInterval::new(1, 150, 250, Strand::Forward);
    /// let c = StrandedGenomicInterval::new(1, 150, 250, Strand::Reverse);
    ///
    /// assert_eq!(a.stranded_overlap_size(&b), Some(50));
    /// assert_eq!(a.stranded_overlap_size(&c), None);
    /// ```
    fn stranded_overlap_size<I: Coordinates<C, T>>(&self, other: &I) -> Option<T> {
        if self.bounded_strand(other) {
            self.overlap_size(other)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 150, 160);
    /// let interval3 = GenomicInterval::new(2, 150, 160);
    ///
    /// assert!(interval1.contains(&interval2));
    /// assert!(!interval1.contains(&interval3));
    /// ```
    fn contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_contains(other)
    }

    /// Returns true if the current interval contains the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)     |---->
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedGenomicInterval, Strand, Coordinates, Overlap};
    ///
    /// let interval1 = StrandedGenomicInterval::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedGenomicInterval::new(1, 150, 160, Strand::Forward);
    /// let interval3 = StrandedGenomicInterval::new(1, 150, 160, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_contains(&interval2));
    /// assert!(!interval1.stranded_contains(&interval3));
    /// ```
    fn stranded_contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.interval_contains(other)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 150, 16);
    /// let interval2 = GenomicInterval::new(1, 100, 200);
    /// let interval3 = GenomicInterval::new(2, 100, 200);
    ///
    /// assert!(interval1.contained_by(&interval2));
    /// assert!(!interval1.contained_by(&interval3));
    /// ```
    fn contained_by<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        other.contains(self)
    }

    /// Returns true if the current interval is contained by the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)      |---->
    /// (Other)   |-------->
    ///
    /// or
    /// (Self)      <----|
    /// (Other)   <--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedGenomicInterval, Strand, Coordinates, Overlap};
    ///
    /// let interval1 = StrandedGenomicInterval::new(1, 150, 160, Strand::Forward);
    /// let interval2 = StrandedGenomicInterval::new(1, 100, 200, Strand::Forward);
    /// let interval3 = StrandedGenomicInterval::new(1, 100, 200, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_contained_by(&interval2));
    /// assert!(!interval1.stranded_contained_by(&interval3));
    /// ```
    fn stranded_contained_by<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        other.stranded_contains(self)
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
    /// use bedrs::{GenomicInterval, Overlap};
    ///
    /// let interval1 = GenomicInterval::new(1, 100, 200);
    /// let interval2 = GenomicInterval::new(1, 200, 300);
    /// let interval3 = GenomicInterval::new(2, 200, 300);
    ///
    /// assert!(interval1.borders(&interval2));
    /// assert!(!interval1.borders(&interval3));
    /// ```
    fn borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_borders(other)
    }

    /// Returns true if the current interval borders the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)            |-------->
    ///
    /// or
    /// (Self)             <--------|
    /// (Other)   <--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedGenomicInterval, Strand, Coordinates, Overlap};
    ///
    /// let interval1 = StrandedGenomicInterval::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedGenomicInterval::new(1, 200, 300, Strand::Forward);
    /// let interval3 = StrandedGenomicInterval::new(1, 200, 300, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_borders(&interval2));
    /// assert!(!interval1.stranded_borders(&interval3));
    /// ```
    fn stranded_borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.borders(other)
    }
}

#[cfg(test)]
mod testing {
    use super::Overlap;
    use crate::{
        types::{record::GenomicInterval, Interval},
        Coordinates, Strand, StrandedGenomicInterval,
    };

    #[test]
    fn test_overlap_self() {
        let a = Interval::new(10, 20);
        assert!(a.overlaps(&a));
    }

    #[test]
    fn test_overlap_reciprocity() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert!(a.overlaps(&b));

        let a = Interval::new(15, 25);
        let b = Interval::new(10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_overlap_negative_reciprocity() {
        let a = Interval::new(10, 20);
        let b = Interval::new(25, 35);
        assert!(!a.overlaps(&b));

        let a = Interval::new(25, 35);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_overlap_boundary() {
        let a = Interval::new(10, 20);
        let b = Interval::new(20, 30);
        assert!(!a.overlaps(&b));
        let a = Interval::new(20, 30);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_self() {
        let a = GenomicInterval::new(1, 10, 20);
        assert!(a.overlaps(&a));
    }

    #[test]
    fn test_genomic_overlap_reciprocity() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(1, 15, 25);
        assert!(a.overlaps(&b));

        let a = GenomicInterval::new(1, 15, 25);
        let b = GenomicInterval::new(1, 10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_negative_reciprocity() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(1, 25, 35);
        assert!(!a.overlaps(&b));

        let a = GenomicInterval::new(1, 25, 35);
        let b = GenomicInterval::new(1, 10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_genomic_overlap_wrong_chr() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(2, 10, 20);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_base_contained() {
        let a = Interval::new(10, 30);
        let b = Interval::new(15, 25);
        let c = Interval::new(10, 30);
        let d = Interval::new(9, 31);
        assert!(a.contains(&b));
        assert!(b.contained_by(&a));
        assert!(a.contains(&c));
        assert!(a.contained_by(&c));
        assert!(!a.contains(&d));
        assert!(a.contained_by(&d));
    }

    #[test]
    fn test_overlapping_contains() {
        let a = Interval::new(10, 30);
        let b = Interval::new(15, 25);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_genomic_contained() {
        let a = GenomicInterval::new(1, 10, 30);
        let b = GenomicInterval::new(1, 15, 25);
        let c = GenomicInterval::new(1, 10, 30);
        let d = GenomicInterval::new(1, 9, 31);
        let e = GenomicInterval::new(2, 15, 25);
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
        let a = Interval::new(10, 20);
        let b = Interval::new(10, 20);
        assert!(a.overlaps(&b));
    }

    #[test]
    fn base_borders() {
        let a = Interval::new(10, 20);
        let b = Interval::new(20, 30);
        assert!(a.borders(&b));
        assert!(b.borders(&a));
    }

    #[test]
    fn genomic_borders() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(1, 20, 30);
        let c = GenomicInterval::new(2, 20, 30);
        assert!(a.borders(&b));
        assert!(b.borders(&a));
        assert!(!a.borders(&c));
        assert!(!c.borders(&a));
    }

    #[test]
    fn overlap_size_lt() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert_eq!(a.overlap_size(&b), Some(5));

        let a = Interval::new(10, 20);
        let b = Interval::new(14, 25);
        assert_eq!(a.overlap_size(&b), Some(6));

        let a = Interval::new(10, 20);
        let b = Interval::new(16, 25);
        assert_eq!(a.overlap_size(&b), Some(4));
    }

    #[test]
    fn overlap_size_gt() {
        let a = Interval::new(15, 25);
        let b = Interval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(5));

        let a = Interval::new(14, 25);
        let b = Interval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(6));

        let a = Interval::new(16, 25);
        let b = Interval::new(10, 20);
        assert_eq!(a.overlap_size(&b), Some(4));
    }

    #[test]
    fn overlap_size_none() {
        let a = Interval::new(10, 20);
        let b = Interval::new(21, 25);
        assert_eq!(a.overlap_size(&b), None);

        let a = Interval::new(21, 25);
        let b = Interval::new(10, 20);
        assert_eq!(a.overlap_size(&b), None);
    }

    #[test]
    fn overlaps_by_lt() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert!(a.overlaps_by(&b, 5));

        let a = Interval::new(10, 20);
        let b = Interval::new(16, 25);
        assert!(!a.overlaps_by(&b, 5));

        let a = Interval::new(10, 20);
        let b = Interval::new(14, 25);
        assert!(a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_by_gt() {
        let a = Interval::new(15, 25);
        let b = Interval::new(10, 20);
        assert!(a.overlaps_by(&b, 5));

        let a = Interval::new(16, 25);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps_by(&b, 5));

        let a = Interval::new(14, 25);
        let b = Interval::new(10, 20);
        assert!(a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_by_none() {
        let a = Interval::new(10, 20);
        let b = Interval::new(21, 25);
        assert!(!a.overlaps_by(&b, 5));

        let a = Interval::new(21, 25);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps_by(&b, 5));
    }

    #[test]
    fn overlaps_exact_lt() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert!(a.overlaps_by_exactly(&b, 5));

        let a = Interval::new(10, 20);
        let b = Interval::new(16, 25);
        assert!(!a.overlaps_by_exactly(&b, 5));

        let a = Interval::new(10, 20);
        let b = Interval::new(14, 25);
        assert!(!a.overlaps_by_exactly(&b, 5));
    }

    #[test]
    fn overlaps_exact_gt() {
        let a = Interval::new(15, 25);
        let b = Interval::new(10, 20);
        assert!(a.overlaps_by_exactly(&b, 5));

        let a = Interval::new(16, 25);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps_by_exactly(&b, 5));

        let a = Interval::new(14, 25);
        let b = Interval::new(10, 20);
        assert!(!a.overlaps_by_exactly(&b, 5));
    }

    #[test]
    fn overlap_size_contains() {
        let a = Interval::new(15, 25);
        let b = Interval::new(17, 23);
        assert_eq!(a.overlap_size(&b), Some(b.len()));
    }

    #[test]
    fn overlap_size_contained_by() {
        let a = Interval::new(17, 23);
        let b = Interval::new(15, 25);
        assert_eq!(a.overlap_size(&b), Some(a.len()));
    }

    #[test]
    fn overlap_stranded() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
        assert!(a.stranded_overlaps(&b));
        assert!(b.stranded_overlaps(&a));
        assert!(!a.stranded_overlaps(&c));
        assert!(!c.stranded_overlaps(&a));
    }

    #[test]
    fn overlap_stranded_borders() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 20, 30, Strand::Reverse);
        assert!(a.stranded_borders(&b));
        assert!(b.stranded_borders(&a));
        assert!(!a.stranded_borders(&c));
        assert!(!c.stranded_borders(&a));
    }

    #[test]
    fn overlap_stranded_contains() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 17, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 17, Strand::Reverse);
        assert!(a.stranded_contains(&b));
        assert!(!b.stranded_contains(&a));
        assert!(!a.stranded_contains(&c));
        assert!(!c.stranded_contains(&a));
    }

    #[test]
    fn overlap_stranded_contained_by() {
        let a = StrandedGenomicInterval::new(1, 15, 17, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 10, 20, Strand::Reverse);
        assert!(a.stranded_contained_by(&b));
        assert!(!b.stranded_contained_by(&a));
        assert!(!a.stranded_contained_by(&c));
        assert!(!c.stranded_contained_by(&a));
    }

    #[test]
    fn overlap_stranded_overlap_size() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
        assert_eq!(a.stranded_overlap_size(&b), Some(5));
        assert_eq!(b.stranded_overlap_size(&a), Some(5));
        assert_eq!(a.stranded_overlap_size(&c), None);
        assert_eq!(c.stranded_overlap_size(&a), None);
    }

    #[test]
    fn stranded_overlaps_by_exactly() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
        assert!(a.stranded_overlaps_by_exactly(&b, 5));
        assert!(b.stranded_overlaps_by_exactly(&a, 5));
        assert!(!a.stranded_overlaps_by_exactly(&c, 5));
        assert!(!c.stranded_overlaps_by_exactly(&a, 5));
    }

    #[test]
    fn stranded_overlaps_by() {
        let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
        assert!(a.stranded_overlaps_by(&b, 5));
        assert!(b.stranded_overlaps_by(&a, 5));
        assert!(!a.stranded_overlaps_by(&c, 5));
        assert!(!c.stranded_overlaps_by(&a, 5));
    }
}
