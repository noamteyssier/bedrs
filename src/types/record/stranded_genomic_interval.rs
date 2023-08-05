use crate::traits::{Coordinates, ValueBounds};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Strand {
    /// The forward strand
    Forward,

    /// The reverse strand
    Reverse,

    /// Unknown strand
    #[default]
    Unknown,
}

/// A representation of a Genomic Interval.
///
/// Has three coordinates: `chr`, `start`, and `end`.
/// Has an associated `Strand` which can be either `Forward` or `Reverse`.
/// This is an associated metadata and is not used in comparisons.
///
/// ```
/// use bedrs::{Coordinates, StrandedGenomicInterval, Overlap, Strand};
///
/// let a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
/// assert_eq!(a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(a.strand(), Strand::Forward);
///
/// let b = StrandedGenomicInterval::new(1, 20, 30, Strand::Reverse);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StrandedGenomicInterval<T> {
    chr: T,
    start: T,
    end: T,
    strand: Strand,
}

impl<T> Coordinates<T> for StrandedGenomicInterval<T>
where
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> T {
        self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &T) {
        self.chr = *val;
    }
    fn from(other: &Self) -> Self {
        Self {
            chr: other.chr(),
            start: other.start(),
            end: other.end(),
            strand: other.strand(),
        }
    }
}

impl<T> StrandedGenomicInterval<T>
where
    T: ValueBounds,
{
    /// Create a new `StrandedGenomicInterval`
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, StrandedGenomicInterval, Strand};
    /// let a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
    /// assert_eq!(a.chr(), 1);
    /// assert_eq!(a.start(), 20);
    /// assert_eq!(a.end(), 30);
    /// assert_eq!(a.strand(), Strand::Forward);
    /// ```
    pub fn new(chr: T, start: T, end: T, strand: Strand) -> Self {
        Self {
            chr,
            start,
            end,
            strand,
        }
    }

    /// Get the strand of the interval
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, StrandedGenomicInterval, Strand};
    ///
    /// let a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
    /// assert_eq!(a.strand(), Strand::Forward);
    /// ```
    pub fn strand(&self) -> Strand {
        self.strand
    }

    /// Set the strand of the interval
    ///
    /// # Examples
    ///
    /// ```
    /// use bedrs::{Coordinates, StrandedGenomicInterval, Strand};
    ///
    /// let mut a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
    /// assert_eq!(a.strand(), Strand::Forward);
    /// a.set_strand(Strand::Reverse);
    /// assert_eq!(a.strand(), Strand::Reverse);
    /// ```
    pub fn set_strand(&mut self, strand: Strand) {
        self.strand = strand;
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::Coordinates,
        types::{Strand, StrandedGenomicInterval},
        Subtract,
    };
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        assert_eq!(interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 10, 90, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = StrandedGenomicInterval::new(2, 10, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(2, 10, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = StrandedGenomicInterval::new(2, 5, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(2, 5, 100, Strand::Forward);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    fn test_set_strand() {
        let mut a = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        assert_eq!(a.strand(), Strand::Forward);
        a.set_strand(Strand::Reverse);
        assert_eq!(a.strand(), Strand::Reverse);
    }

    #[test]
    fn test_subtraction_a() {
        let a = StrandedGenomicInterval::new(1, 5, 100, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_b() {
        let a = StrandedGenomicInterval::new(1, 5, 100, Strand::Reverse);
        let b = StrandedGenomicInterval::new(1, 10, 100, Strand::Forward);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand(), Strand::Forward);
    }

    #[test]
    fn test_subtraction_c() {
        let a = StrandedGenomicInterval::new(1, 5, 100, Strand::Reverse);
        let b = StrandedGenomicInterval::new(1, 10, 100, Strand::Reverse);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 5);
        assert_eq!(sub[0].end(), 10);
        assert_eq!(sub[0].strand(), Strand::Reverse);
    }
}
