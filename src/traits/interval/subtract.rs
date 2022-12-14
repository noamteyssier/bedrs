use crate::{traits::ValueBounds, Coordinates, Overlap};

/// Trait for performing subtraction with coordinates
pub trait Subtract<T>: Coordinates<T> + Overlap<T>
where
    T: ValueBounds,
{
    fn build_left_contained<I: Coordinates<T>>(&self, other: &I) -> I {
        let left_start = self.start().min(other.start());
        let left_end = self.start().max(other.start());
        let mut left_sub = I::from(other);
        left_sub.update_all(&other.chr(), &left_start, &left_end);
        left_sub
    }
    fn build_right_contained<I: Coordinates<T>>(&self, other: &I) -> I {
        let right_start = self.end().min(other.end());
        let right_end = self.end().max(other.end());
        let mut right_sub = I::from(other);
        right_sub.update_all(&other.chr(), &right_start, &right_end);
        right_sub
    }
    fn build_contained<I: Coordinates<T>>(&self, other: &I) -> Vec<I> {
        vec![
            self.build_left_contained(other),
            self.build_right_contained(other),
        ]
    }
    fn build_gt<I: Coordinates<T>>(&self, other: &I) -> Vec<I> {
        let mut sub = I::from(other);
        sub.update_all(&other.chr(), &other.end(), &self.end());
        vec![sub]
    }
    fn build_lt<I: Coordinates<T>>(&self, other: &I) -> Vec<I> {
        let mut sub = I::from(other);
        sub.update_all(&other.chr(), &self.start(), &other.start());
        vec![sub]
    }
    fn build_self<I: Coordinates<T>>(&self, other: &I) -> Vec<I> {
        let mut sub = I::from(&other);
        sub.update_all(&other.chr(), &self.start(), &self.end());
        vec![sub]
    }
    /// Perform subtraction between two coordinates.
    ///
    /// Returns a vector of intersections, as depending on the
    /// containment status there could either zero, one, or two
    /// subtraction intervals for any overlapping intervals.
    ///
    /// ## Left Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)       x-------y
    /// // (b)   i-----j
    /// // ======================
    /// // (s)         j----y
    ///
    /// let a = Interval::new(20, 30);
    /// let b = Interval::new(15, 25);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 25);
    /// assert_eq!(s[0].end(), 30);
    /// ```
    ///
    /// ## Right Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)   x-----y
    /// // (b)       i-------j
    /// // =======================
    /// // (s)   x---i
    ///
    /// let a = Interval::new(15, 25);
    /// let b = Interval::new(20, 30);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 15);
    /// assert_eq!(s[0].end(), 20);
    /// ```
    ///
    /// ## Contains
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)   x-----------y
    /// // (b)       i--j
    /// // =======================
    /// // (s)   x---i  j----y
    ///
    /// let a = Interval::new(10, 40);
    /// let b = Interval::new(20, 30);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 2);
    /// assert_eq!(s[0].start(), 10);
    /// assert_eq!(s[0].end(), 20);
    /// assert_eq!(s[1].start(), 30);
    /// assert_eq!(s[1].end(), 40);
    /// ```
    ///
    /// ## Contained by
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)       x--y
    /// // (b)   i-----------j
    /// // =======================
    /// // (s) None
    ///
    /// let a = Interval::new(20, 30);
    /// let b = Interval::new(10, 40);
    /// let s = a.subtract(&b);
    /// assert!(s.is_none());
    /// ```
    ///
    /// ## Complete Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)       x--y
    /// // (b)       i--j
    /// // =======================
    /// // (s) None
    ///
    /// let a = Interval::new(10, 30);
    /// let b = Interval::new(10, 30);
    /// let s = a.subtract(&b);
    /// assert!(s.is_none());
    /// ```
    ///
    /// ## No Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, Interval};
    ///
    /// // (a)  x--y
    /// // (b)       i--j
    /// // =======================
    /// // (s)  x--y
    ///
    /// let a = Interval::new(10, 20);
    /// let b = Interval::new(30, 40);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 10);
    /// assert_eq!(s[0].end(), 20);
    /// ```
    fn subtract<I: Coordinates<T>>(&self, other: &I) -> Option<Vec<I>> {
        if self.overlaps(other) {
            if self.eq(other) || self.contained_by(other) {
                None
            } else if self.contains(other) || self.contained_by(other) {
                Some(self.build_contained(other))
            } else if self.gt(other) {
                Some(self.build_gt(other))
            } else if self.lt(other) {
                Some(self.build_lt(other))
            } else {
                todo!()
            }
        } else {
            Some(self.build_self(other))
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Subtract;
    use crate::{Coordinates, GenomicInterval, Interval};

    #[test]
    ///      x-------y
    ///   i-----j
    /// ==================
    ///         j----y
    fn subtraction_case_a() {
        let a = Interval::new(20, 30);
        let b = Interval::new(15, 25);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 25);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    ///      x-------y
    ///   i-----j
    /// ==================
    ///         j----y
    fn subtraction_genomic_a() {
        let a = GenomicInterval::new(1, 20, 30);
        let b = GenomicInterval::new(1, 15, 25);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 25);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    ///   x-----y
    ///      i-------j
    /// ==================
    ///   x--i
    fn subtraction_case_b() {
        let a = Interval::new(15, 25);
        let b = Interval::new(20, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 15);
        assert_eq!(sub[0].end(), 20);
    }

    #[test]
    ///   x------y
    ///     i--j
    /// ==================
    ///   x-i  j-y
    fn subtraction_case_c() {
        let a = Interval::new(10, 40);
        let b = Interval::new(20, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 2);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 20);
        assert_eq!(sub[1].start(), 30);
        assert_eq!(sub[1].end(), 40);
    }

    #[test]
    ///     x--y
    ///   i------j
    /// ==================
    ///   i-x  y-j
    fn subtraction_case_d() {
        let a = Interval::new(20, 30);
        let b = Interval::new(10, 40);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    ///     x--y
    ///     i--j
    /// ==================
    /// none
    fn subtraction_case_e() {
        let a = Interval::new(10, 30);
        let b = Interval::new(10, 30);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    ///     x--y
    ///     i--j
    /// ==================
    /// none
    fn subtraction_genomic_e() {
        let a = GenomicInterval::new(1, 10, 30);
        let b = GenomicInterval::new(1, 10, 30);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    ///     x--y  <- chr1
    ///     i--j  <- chr2
    /// ==================
    ///     x--y
    fn subtraction_genomic_e_wrong_chr() {
        let a = GenomicInterval::new(1, 10, 30);
        let b = GenomicInterval::new(2, 10, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    ///   x--y
    ///        i--j
    /// ==================
    ///   x--y
    fn subtraction_case_f() {
        let a = Interval::new(10, 20);
        let b = Interval::new(30, 40);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 20);
    }
}
