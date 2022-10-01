use crate::{traits::ValueBounds, Overlap, Coordinates};

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
        let start = other.end();
        let end = self.end();
        let mut sub = I::from(other);
        sub.update_all(&other.chr(), &start, &end);
        vec![sub]
    }
    fn build_lt<I: Coordinates<T>>(&self, other: &I) -> Vec<I> {
        let start = self.start();
        let end = other.start();
        let mut sub = I::from(other);
        sub.update_all(&other.chr(), &start, &end);
        vec![sub]
    }
    fn subtract<I: Coordinates<T>>(&self, other: &I) -> Option<Vec<I>> {
        if self.overlaps(other) {
            if self.eq(other) {
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
            None
        }

    }
}

#[cfg(test)]
mod testing {
    use crate::{Interval, Coordinates};
    use super::Subtract;


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
        let sub = a.subtract(&b).unwrap();
        assert_eq!(sub.len(), 2);
        assert_eq!(sub[0].start(), 10);
        assert_eq!(sub[0].end(), 20);
        assert_eq!(sub[1].start(), 30);
        assert_eq!(sub[1].end(), 40);
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
    ///   x--y
    ///        i--j
    /// ==================
    /// none
    fn subtraction_case_f() {
        let a = Interval::new(10, 20);
        let b = Interval::new(30, 40);
        let sub = a.subtract(&b);
        assert!(sub.is_none());
    }
}
