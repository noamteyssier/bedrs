use crate::{
    traits::{ChromBounds, ValueBounds},
    Coordinates, Overlap,
};

/// Calculates the distance between two coordinates.
///
/// Only works for coordinates that are on the same chromosome
/// as there is no notion of chromosomal distance.
///
/// # Example
///
/// ## Unsigned distance
///
/// This distance metric is unsigned and will always return a positive value.
/// It is symmetric between the two coordinates.
///
/// ```
/// use bedrs::*;
///
/// let a = BaseInterval::new(10, 20);
/// let b = BaseInterval::new(30, 40);
/// assert_eq!(a.distance(&b), Some(10));
/// assert_eq!(b.distance(&a), Some(10));
/// ```
///
/// ## Signed distance
///
/// This distance metric is signed and will return a positive or negative value.
/// It is not symmetric between the two coordinates.
///
/// A positive value indicates that the first coordinate is upstream of the second.
/// A negative value indicates that the first coordinate is downstream of the second.
///
/// ```
/// use bedrs::*;
///
/// let a = BaseInterval::new(10, 20);
/// let b = BaseInterval::new(30, 40);
/// assert_eq!(a.directed_distance(&b), Some(10));
/// assert_eq!(b.directed_distance(&a), Some(-10));
/// ```
///
/// ## No distance
///
/// If the two coordinates overlap or border each other, the distance is zero.
/// If the two coordinates are on different chromosomes, the distance is undefined.
///
/// ```
/// use bedrs::*;
///
/// // Bordering Intervals
/// let a = BaseInterval::new(10, 20);
/// let b = BaseInterval::new(20, 30);
/// assert_eq!(a.distance(&b), Some(0));
///
/// // Overlapping Intervals
/// let a = BaseInterval::new(10, 20);
/// let b = BaseInterval::new(18, 30);
/// assert_eq!(a.distance(&b), Some(0));
///
/// // Different Chromosomes
/// let a = Bed3::new(1, 10, 20);
/// let b = Bed3::new(2, 10, 20);
/// assert_eq!(a.distance(&b), None);
/// ```
pub trait Distance<C>: Coordinates<C> + Overlap<C>
where
    C: ChromBounds,
    i32: ValueBounds,
{
    fn distance<I: Coordinates<C>>(&self, other: &I) -> Option<i32> {
        if self.overlaps(other) || self.borders(other) {
            Some(0)
        } else if self.chr() != other.chr() {
            None
        } else if self.gt(other) {
            Some(self.start() - other.end())
        } else {
            Some(other.start() - self.end())
        }
    }

    fn directed_distance<I: Coordinates<C>>(&self, other: &I) -> Option<i32> {
        if self.overlaps(other) || self.borders(other) {
            Some(0)
        } else if self.chr() != other.chr() {
            None
        } else {
            // always signed
            Some(self.start() - other.end())
        }
    }
}

#[cfg(test)]
#[allow(clippy::doc_markdown)]
mod testing {
    use crate::{traits::interval::Distance, BaseInterval, Bed3};

    #[test]
    ///    x-----y
    ///       x-----y
    /// ================
    /// distance = 0
    fn distance_a() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert_eq!(a.distance(&b), Some(0));
    }

    #[test]
    ///    x-----y
    ///          x-----y
    /// ===================
    /// distance = 0
    fn distance_b() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(20, 30);
        assert_eq!(a.distance(&b), Some(0));
    }

    #[test]
    ///   x------y
    ///           x-----y
    /// ===================
    /// distance = 1
    fn distance_c() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(21, 30);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    ///           x-----y
    ///   x------y
    /// ===================
    /// distance = 1
    fn distance_d() {
        let a = BaseInterval::new(21, 30);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    /// |1|          x-----y
    /// |1|  x------y
    /// ===================
    /// distance = 1
    fn distance_e() {
        let a = Bed3::new(1, 21, 30);
        let b = Bed3::new(1, 10, 20);
        assert_eq!(a.distance(&b), Some(1));
    }

    #[test]
    /// |2|          x-----y
    /// |1|  x------y
    /// ===================
    /// distance = None
    fn distance_f() {
        let a = Bed3::new(2, 21, 30);
        let b = Bed3::new(1, 10, 20);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |1|  x------y
    /// |2|          x-----y
    /// ===================
    /// distance = None
    fn distance_g() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(2, 21, 30);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |2|  x------y
    /// |1|          x-----y
    /// ===================
    /// distance = None
    fn distance_h() {
        let a = Bed3::new(2, 10, 20);
        let b = Bed3::new(1, 21, 30);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    /// |1|          x-----y
    /// |2|  x------y
    /// ===================
    /// distance = None
    fn distance_i() {
        let a = Bed3::new(2, 21, 30);
        let b = Bed3::new(1, 10, 20);
        assert_eq!(a.distance(&b), None);
    }

    #[test]
    ///    x-----y
    ///       x-----y
    /// ================
    /// directed_distance = 0
    fn directed_distance_a() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(15, 25);
        assert_eq!(a.directed_distance(&b), Some(0));
    }

    #[test]
    ///    x-----y
    ///          x-----y
    /// ===================
    /// directed_distance = 0
    fn directed_distance_b() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(20, 30);
        assert_eq!(a.directed_distance(&b), Some(0));
    }

    #[test]
    ///   x------y
    ///           x-----y
    /// ===================
    /// directed_distance = 1
    fn directed_distance_c() {
        let a = BaseInterval::new(10, 20);
        let b = BaseInterval::new(21, 30);
        assert_eq!(a.directed_distance(&b), Some(1));
    }

    #[test]
    ///           x-----y
    ///   x------y
    /// ===================
    /// directed_distance = -1
    fn directed_distance_d() {
        let a = BaseInterval::new(21, 30);
        let b = BaseInterval::new(10, 20);
        assert_eq!(a.directed_distance(&b), Some(-1));
    }

    #[test]
    /// |2|          x-----y
    /// |1|  x------y
    /// ===================
    /// directed_distance = None
    fn directed_distance_e() {
        let a = Bed3::new(2, 21, 30);
        let b = Bed3::new(1, 10, 20);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |1|  x------y
    /// |2|          x-----y
    /// ===================
    /// directed_distance = None
    fn directed_distance_f() {
        let a = Bed3::new(1, 10, 20);
        let b = Bed3::new(2, 21, 30);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |2|  x------y
    /// |1|          x-----y
    /// ===================
    /// directed_distance = None
    fn directed_distance_g() {
        let a = Bed3::new(2, 10, 20);
        let b = Bed3::new(1, 21, 30);
        assert_eq!(a.directed_distance(&b), None);
    }

    #[test]
    /// |1|          x-----y
    /// |2|  x------y
    /// ===================
    /// directed_distance = None
    fn directed_distance_h() {
        let a = Bed3::new(2, 21, 30);
        let b = Bed3::new(1, 10, 20);
        assert_eq!(a.directed_distance(&b), None);
    }
}
