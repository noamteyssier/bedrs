use super::Coordinates;

pub trait Overlap<T: PartialOrd>: Coordinates<T> {
    fn overlaps(&self, other: &Self) -> bool {
        (other.start() >= self.start() && other.start() <= self.end())
            || (other.end() >= self.start() && (other.end() <= self.end()))
    }
}

#[cfg(test)]
mod testing {
    use super::Overlap;
    use crate::types::Interval;

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
}
