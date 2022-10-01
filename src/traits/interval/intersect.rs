use crate::{traits::ValueBounds, Coordinates, Overlap};

/// Calculates the intersection between two coordinates.
pub trait Intersect<T>: Coordinates<T> + Overlap<T>
where
    T: ValueBounds,
{
    fn build_intersection_interval<I: Coordinates<T>>(&self, other: &I) -> I {
        let chr = self.chr();
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        let mut interval = I::from(other);
        interval.update_all(&chr, &start, &end);
        interval
    }

    /// Calculates the intersection between two coordinates if they overlap.
    ///
    /// ```
    /// use bedrs::{Coordinates, Intersect, Interval};
    ///
    /// let a = Interval::new(10, 20);
    /// let b = Interval::new(15, 25);
    /// let ix = a.intersect(&b).unwrap();
    ///
    /// assert_eq!(ix.start(), 15);
    /// assert_eq!(ix.end(), 20);
    /// ```
    fn intersect<I: Coordinates<T>>(&self, other: &I) -> Option<I> {
        if self.overlaps(other) {
            let ix = self.build_intersection_interval(other);
            Some(ix)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Intersect;
    use crate::{Coordinates, Interval};

    #[test]
    ///       x-------y
    ///    i------j
    /// =====================
    ///       x---j
    fn intersection_case_a() {
        let a = Interval::new(20, 30);
        let b = Interval::new(15, 25);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
    }

    #[test]
    ///    x-------y
    ///        i------j
    /// ==================
    ///        i---y
    fn intersection_case_b() {
        let a = Interval::new(20, 30);
        let b = Interval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
    }

    #[test]
    ///    x--------y
    ///       i--j
    /// ==================
    ///       i--j
    fn intersection_case_c() {
        let a = Interval::new(20, 40);
        let b = Interval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    ///       x--y
    ///    i--------j
    /// ==================
    ///       x--y
    fn intersection_case_d() {
        let a = Interval::new(25, 35);
        let b = Interval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    ///       x--y
    ///       i--j
    /// ==================
    ///       x--y
    fn intersection_case_e() {
        let a = Interval::new(20, 40);
        let b = Interval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
    }

    #[test]
    ///    x--y
    ///         i--j
    /// ==================
    ///       none
    fn intersection_case_none() {
        let a = Interval::new(10, 15);
        let b = Interval::new(20, 25);
        let ix = a.intersect(&b);
        assert!(ix.is_none());
    }
}
