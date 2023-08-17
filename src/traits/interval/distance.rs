use crate::{
    traits::{ChromBounds, ValueBounds},
    Coordinates, Overlap,
};

/// Calculates the distance between two coordinates.
pub trait Distance<C, T>: Coordinates<C, T> + Overlap<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn distance<I: Coordinates<C, T>>(&self, other: &I) -> Option<T> {
        if self.overlaps(other) || self.borders(other) {
            Some(T::zero())
        } else if self.chr() != other.chr() {
            None
        } else if self.gt(other) {
            Some(self.start() - other.end())
        } else {
            Some(other.start() - self.end())
        }
    }

    fn directed_distance<I: Coordinates<C, T>>(&self, other: &I) -> Option<isize> {
        if self.overlaps(other) || self.borders(other) {
            Some(0)
        } else if self.chr() != other.chr() {
            None
        } else if self.gt(other) {
            (self.start() - other.end()).to_isize().map(|x| -x)
        } else {
            (other.start() - self.end()).to_isize()
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{traits::interval::Distance, GenomicInterval, Interval};

    #[test]
    ///    x-----y
    ///       x-----y
    /// ================
    /// distance = 0
    fn distance_a() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert_eq!(a.distance(&b), Some(0));
    }

    #[test]
    ///    x-----y
    ///          x-----y
    /// ===================
    /// distance = 0
    fn distance_b() {
        let a = Interval::new(10, 20);
        let b = Interval::new(20, 30);
        assert_eq!(a.distance(&b), Some(0));
    }

    #[test]
    ///   x------y
    ///           x-----y
    /// ===================
    /// distance = 1
    fn distance_c() {
        let a = Interval::new(10, 20);
        let b = Interval::new(21, 30);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    ///           x-----y
    ///   x------y
    /// ===================
    /// distance = 1
    fn distance_d() {
        let a = Interval::new(21, 30);
        let b = Interval::new(10, 20);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    /// |1|          x-----y
    /// |1|  x------y
    /// ===================
    /// distance = 1
    fn distance_e() {
        let a = GenomicInterval::new(1, 21, 30);
        let b = GenomicInterval::new(1, 10, 20);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    /// |2|          x-----y
    /// |1|  x------y
    /// ===================
    /// distance = None
    fn distance_f() {
        let a = GenomicInterval::new(2, 21, 30);
        let b = GenomicInterval::new(1, 10, 20);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |1|  x------y
    /// |2|          x-----y
    /// ===================
    /// distance = None
    fn distance_g() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(2, 21, 30);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |2|  x------y
    /// |1|          x-----y
    /// ===================
    /// distance = None
    fn distance_h() {
        let a = GenomicInterval::new(2, 10, 20);
        let b = GenomicInterval::new(1, 21, 30);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |1|          x-----y
    /// |2|  x------y
    /// ===================
    /// distance = None
    fn distance_i() {
        let a = GenomicInterval::new(2, 21, 30);
        let b = GenomicInterval::new(1, 10, 20);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    ///    x-----y
    ///       x-----y
    /// ================
    /// directed_distance = 0
    fn directed_distance_a() {
        let a = Interval::new(10, 20);
        let b = Interval::new(15, 25);
        assert_eq!(a.directed_distance(&b), Some(0));
    }

    #[test]
    ///    x-----y
    ///          x-----y
    /// ===================
    /// directed_distance = 0
    fn directed_distance_b() {
        let a = Interval::new(10, 20);
        let b = Interval::new(20, 30);
        assert_eq!(a.directed_distance(&b), Some(0));
    }

    #[test]
    ///   x------y
    ///           x-----y
    /// ===================
    /// directed_distance = 1
    fn directed_distance_c() {
        let a = Interval::new(10, 20);
        let b = Interval::new(21, 30);
        assert_eq!(a.directed_distance(&b), Some(1));
    }

    #[test]
    ///           x-----y
    ///   x------y
    /// ===================
    /// directed_distance = -1
    fn directed_distance_d() {
        let a = Interval::new(21, 30);
        let b = Interval::new(10, 20);
        assert_eq!(a.directed_distance(&b), Some(-1));
    }

    #[test]
    /// |2|          x-----y
    /// |1|  x------y
    /// ===================
    /// directed_distance = None
    fn directed_distance_e() {
        let a = GenomicInterval::new(2, 21, 30);
        let b = GenomicInterval::new(1, 10, 20);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |1|  x------y
    /// |2|          x-----y
    /// ===================
    /// directed_distance = None
    fn directed_distance_f() {
        let a = GenomicInterval::new(1, 10, 20);
        let b = GenomicInterval::new(2, 21, 30);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |2|  x------y
    /// |1|          x-----y
    /// ===================
    /// directed_distance = None
    fn directed_distance_g() {
        let a = GenomicInterval::new(2, 10, 20);
        let b = GenomicInterval::new(1, 21, 30);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |1|          x-----y
    /// |2|  x------y
    /// ===================
    /// directed_distance = None
    fn directed_distance_h() {
        let a = GenomicInterval::new(2, 21, 30);
        let b = GenomicInterval::new(1, 10, 20);
        assert_eq!(a.directed_distance(&b), None);
    }
}
