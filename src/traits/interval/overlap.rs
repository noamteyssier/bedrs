use super::Coordinates;
use crate::traits::ValueBounds;

/// A trait to measure overlaps between intervals implementing `Coordinates`
pub trait Overlap<T>: Coordinates<T>
where
    Self: Sized,
    T: ValueBounds,
{
    /// Returns true if the two intervals are on the same chromosome.
    ///
    /// ``` text
    /// (Self)   chr1:  |---------|
    /// (Other)  chr2:                |---------|
    /// ```
    fn bounded_chr<I: Coordinates<T>>(&self, other: &I) -> bool {
        other.chr() == self.chr()
    }

    /// Returns true if the two intervals overlap.
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
    fn interval_overlap<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }

    /// Returns true if the current interval contains the other interval.
    ///
    /// Measured as bool and of:
    /// * `self.start` is less than `other.start`
    /// * `self.end` is greater than `other.end`
    ///
    /// ```text
    /// (Self)    |--------|
    /// (Other)     |----|
    /// ```
    fn interval_contains<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.start() < other.start() && self.end() > other.end()
    }

    /// Returns true if the current interval borders the other interval.
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
    fn interval_borders<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.start().eq(&other.end()) || self.end().eq(&other.start())
    }

    /// Returns true if the current interval overlaps the other -
    /// considers both the interval overlap and the chromosome.
    fn overlaps<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_overlap(other)
    }

    /// Returns true if the current interval contains the other -
    /// considers both the interval overlap and the chromosome.
    fn contains<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_contains(other)
    }

    /// Returns true if the current interval is contained by the other -
    /// considers both the interval overlap and the chromosome.
    fn contained_by<I: Coordinates<T>>(&self, other: &I) -> bool {
        other.contains(self)
    }

    /// Returns true if the current interval borders the other -
    /// considers both the interval overlap and the chromosome.
    fn borders<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && self.interval_borders(other)
    }
}

#[cfg(test)]
mod testing {
    use super::Overlap;
    use crate::types::{record::GenomicInterval, Interval};

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
        assert!(!a.contains(&c));
        assert!(!a.contained_by(&c));
        assert!(!a.contains(&d));
        assert!(a.contained_by(&d));
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
        assert!(!a.contains(&c));
        assert!(!a.contained_by(&c));
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
}
