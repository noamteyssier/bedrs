use super::Coordinates;

pub trait Overlap<T>: Coordinates<T>
where
    T: Copy + Default + PartialOrd,
{
    fn bounded_start<I: Coordinates<T>>(&self, other: &I) -> bool {
        other.start() >= self.start() && other.start() <= self.end()
    }
    fn bounded_end<I: Coordinates<T>>(&self, other: &I) -> bool {
        other.end() >= self.start() && other.end() <= self.end()
    }
    fn bounded_chr<I: Coordinates<T>>(&self, other: &I) -> bool {
        other.chr() == self.chr()
    }
    fn overlaps<I: Coordinates<T>>(&self, other: &I) -> bool {
        self.bounded_chr(other) && (
            self.bounded_start(other) || self.bounded_end(other)
        )
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
}
