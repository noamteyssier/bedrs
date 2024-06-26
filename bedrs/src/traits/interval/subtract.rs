use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Coordinates, Overlap,
};

/// Trait for performing subtraction with coordinates
pub trait Subtract<C, T>: Coordinates<C, T> + Overlap<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    #[must_use]
    fn build_left_contained<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let left_start = self.start().min(other.start());
        let left_end = self.start().max(other.start());
        let mut left_sub = Self::from(other);
        left_sub.update_all(other.chr(), &left_start, &left_end);
        left_sub
    }
    #[must_use]
    fn build_right_contained<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let right_start = self.end().min(other.end());
        let right_end = self.end().max(other.end());
        let mut right_sub = Self::from(other);
        right_sub.update_all(other.chr(), &right_start, &right_end);
        right_sub
    }
    fn build_contained_iter<I: Coordinates<C, T>>(
        &self,
        other: &I,
    ) -> Box<dyn Iterator<Item = Self>>
    where
        Self: 'static,
    {
        if self.start() == other.start() {
            let iter = std::iter::once(self.build_right_contained(other));
            Box::new(iter)
        } else if self.end() == other.end() {
            let iter = std::iter::once(self.build_left_contained(other));
            Box::new(iter)
        } else {
            let iter = std::iter::once(self.build_left_contained(other))
                .chain(std::iter::once(self.build_right_contained(other)));
            Box::new(iter)
        }
    }
    #[must_use]
    fn build_gt<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let mut sub = Self::from(other);
        sub.update_all(other.chr(), &other.end(), &self.end());
        sub
    }
    #[must_use]
    fn build_lt<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let mut sub = Self::from(other);
        sub.update_all(other.chr(), &self.start(), &other.start());
        sub
    }
    #[must_use]
    fn build_self<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let mut sub = Self::from(other);
        sub.update_all(other.chr(), &self.start(), &self.end());
        sub
    }
    /// Perform subtraction between two coordinates.
    ///
    /// Returns a vector of intersections, as depending on the
    /// containment status there could either zero, one, or two
    /// subtraction intervals for any overlapping intervals.
    ///
    /// ## Left Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)       x-------y
    /// // (b)   i-----j
    /// // ======================
    /// // (s)         j----y
    ///
    /// let a = BaseInterval::new(20, 30);
    /// let b = BaseInterval::new(15, 25);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 25);
    /// assert_eq!(s[0].end(), 30);
    /// ```
    ///
    /// ## Right Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)   x-----y
    /// // (b)       i-------j
    /// // =======================
    /// // (s)   x---i
    ///
    /// let a = BaseInterval::new(15, 25);
    /// let b = BaseInterval::new(20, 30);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 15);
    /// assert_eq!(s[0].end(), 20);
    /// ```
    ///
    /// ## Contains
    /// ```
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)   x-----------y
    /// // (b)       i--j
    /// // =======================
    /// // (s)   x---i  j----y
    ///
    /// let a = BaseInterval::new(10, 40);
    /// let b = BaseInterval::new(20, 30);
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
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)       x--y
    /// // (b)   i-----------j
    /// // =======================
    /// // (s) None
    ///
    /// let a = BaseInterval::new(20, 30);
    /// let b = BaseInterval::new(10, 40);
    /// let s = a.subtract(&b);
    /// assert!(s.is_none());
    /// ```
    ///
    /// ## Complete Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)       x--y
    /// // (b)       i--j
    /// // =======================
    /// // (s) None
    ///
    /// let a = BaseInterval::new(10, 30);
    /// let b = BaseInterval::new(10, 30);
    /// let s = a.subtract(&b);
    /// assert!(s.is_none());
    /// ```
    ///
    /// ## No Overlap
    /// ```
    /// use bedrs::{Coordinates, Subtract, BaseInterval};
    ///
    /// // (a)  x--y
    /// // (b)       i--j
    /// // =======================
    /// // (s)  x--y
    ///
    /// let a = BaseInterval::new(10, 20);
    /// let b = BaseInterval::new(30, 40);
    /// let s = a.subtract(&b).unwrap();
    /// assert_eq!(s.len(), 1);
    /// assert_eq!(s[0].start(), 10);
    /// assert_eq!(s[0].end(), 20);
    /// ```
    fn subtract<I: Coordinates<C, T>>(&self, other: &I) -> Option<Vec<Self>> {
        if self.overlaps(other) {
            if self.eq(other) || self.contained_by(other) {
                None
            } else if self.contains(other) {
                if self.start() == other.start() {
                    Some(vec![self.build_gt(other)])
                } else if self.end() == other.end() {
                    Some(vec![self.build_lt(other)])
                } else {
                    let left = self.build_lt(other);
                    let right = self.build_gt(other);
                    Some(vec![left, right])
                }
            } else if self.gt(other) {
                Some(vec![self.build_gt(other)])
            } else {
                Some(vec![self.build_lt(other)])
            }
        } else {
            Some(vec![self.build_self(other)])
        }
    }

    fn subtract_iter<I: IntervalBounds<C, T>>(&self, other: &I) -> Box<dyn Iterator<Item = Self>>
    where
        Self: 'static,
    {
        if self.overlaps(other) {
            if self.eq(other) || self.contained_by(other) {
                Box::new(std::iter::empty())
            } else if self.contains(other) {
                self.build_contained_iter(other)
            } else if self.gt(other) {
                let iv = self.build_gt(other);
                Box::new(std::iter::once(iv))
            } else {
                let iv = self.build_lt(other);
                Box::new(std::iter::once(iv))
            }
        } else {
            let iv = self.build_self(other);
            Box::new(std::iter::once(iv))
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Subtract;
    use crate::{BaseInterval, Bed3, Coordinates};

    #[test]
    ///      x-------y
    ///   i-----j
    /// ==================
    ///         j----y
    fn subtraction_case_a() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(15, 25);
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
        let a = Bed3::new(1, 20, 30);
        let b = Bed3::new(1, 15, 25);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 25);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    fn subtraction_case_a_iter() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(15, 25);
        let mut sub = a.subtract_iter(&b);
        assert_eq!(sub.next().unwrap().start(), 25);
        assert!(sub.next().is_none());
    }

    #[test]
    ///   x-----y
    ///      i-------j
    /// ==================
    ///   x--i
    fn subtraction_case_b() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(20, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 15);
        assert_eq!(sub[0].end(), 20);
    }

    #[test]
    fn subtraction_case_b_iter() {
        let a = BaseInterval::new(15, 25);
        let b = BaseInterval::new(20, 30);
        let mut sub = a.subtract_iter(&b);
        assert_eq!(sub.next().unwrap().start(), 15);
        assert!(sub.next().is_none());
    }

    #[test]
    ///   x------y
    ///     i--j
    /// ==================
    ///   x-i  j-y
    fn subtraction_case_c() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(20, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 2);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 20);
        assert_eq!(sub[1].start(), 30);
        assert_eq!(sub[1].end(), 40);
    }

    #[test]
    fn subtraction_case_c_iter() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(20, 30);
        let mut sub = a.subtract_iter(&b);
        assert_eq!(sub.next().unwrap().start(), 10);
        assert_eq!(sub.next().unwrap().start(), 30);
        assert!(sub.next().is_none());
    }

    #[test]
    ///     x--y
    ///   i------j
    /// ==================
    ///   none
    fn subtraction_case_d() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(10, 40);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    fn subtraction_case_d_iter() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(10, 40);
        let mut sub = a.subtract_iter(&b);
        assert!(sub.next().is_none());
    }

    #[test]
    ///     x--y
    ///     i--j
    /// ==================
    /// none
    fn subtraction_case_e() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(10, 30);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    fn subtraction_case_e_iter() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(10, 30);
        let mut sub = a.subtract_iter(&b);
        assert!(sub.next().is_none());
    }

    #[test]
    ///     x--y
    ///     i--j
    /// ==================
    /// none
    fn subtraction_genomic_e() {
        let a = Bed3::new(1, 10, 30);
        let b = Bed3::new(1, 10, 30);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    fn subtraction_genomic_e_iter() {
        let a = Bed3::new(1, 10, 30);
        let b = Bed3::new(1, 10, 30);
        let mut sub = a.subtract_iter(&b);
        assert!(sub.next().is_none());
    }

    #[test]
    ///     x--y  <- chr1
    ///     i--j  <- chr2
    /// ==================
    ///     x--y
    fn subtraction_genomic_e_wrong_chr() {
        let a = Bed3::new(1, 10, 30);
        let b = Bed3::new(2, 10, 30);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    fn subtraction_genomic_e_wrong_chr_iter() {
        let a = Bed3::new(1, 10, 30);
        let b = Bed3::new(2, 10, 30);
        let mut sub = a.subtract_iter(&b);
        let first = sub.next().unwrap();
        assert_eq!(first.start(), 10);
        assert_eq!(first.end(), 30);
        assert!(sub.next().is_none());
    }

    #[test]
    ///   x--y
    ///        i--j
    /// ==================
    ///   x--y
    fn subtraction_case_f() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(30, 40);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 20);
    }

    #[test]
    fn subtraction_case_f_iter() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(30, 40);
        let mut sub = a.subtract_iter(&b);
        let first = sub.next().unwrap();
        assert_eq!(first.start(), 10);
        assert_eq!(first.end(), 20);
        assert!(sub.next().is_none());
    }

    #[test]
    ///   x--------y
    ///   i---j
    /// ===============
    ///       j----y
    fn subtraction_case_g() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(10, 20);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 20);
        assert_eq!(sub[0].end(), 40);
    }

    #[test]
    fn subtraction_case_g_iter() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(10, 20);
        let mut sub = a.subtract_iter(&b);
        let first = sub.next().unwrap();
        assert_eq!(first.start(), 20);
        assert_eq!(first.end(), 40);
        assert!(sub.next().is_none());
    }

    #[test]
    ///   x--------y
    ///        i---j
    /// ===============
    ///   x----i
    fn subtraction_case_h() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(30, 40);
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 30);
    }

    #[test]
    fn subtraction_case_h_iter() {
        let a = BaseInterval::new(10, 40);
        let b = BaseInterval::new(30, 40);
        let mut sub = a.subtract_iter(&b);
        let first = sub.next().unwrap();
        assert_eq!(first.start(), 10);
        assert_eq!(first.end(), 30);
        assert!(sub.next().is_none());
    }

    #[test]
    ///     x-----y
    ///  i--------j   
    /// ===============
    /// none
    fn subtraction_case_i() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(10, 30);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    fn subtraction_case_i_iter() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(10, 30);
        let mut sub = a.subtract_iter(&b);
        assert!(sub.next().is_none());
    }

    #[test]
    ///  x-----y
    ///  i--------j   
    /// ===============
    /// none
    fn subtraction_case_j() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(10, 40);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }

    #[test]
    fn subtraction_case_j_iter() {
        let a = BaseInterval::new(10, 30);
        let b = BaseInterval::new(10, 40);
        let mut sub = a.subtract_iter(&b);
        assert!(sub.next().is_none());
    }
}
