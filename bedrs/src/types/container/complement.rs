use anyhow::{bail, Result};

use crate::{
    traits::{ChromBounds, IntervalBounds, SetError},
    types::{iterator::ComplementIter, IntervalIterOwned},
    IntervalContainer,
};

type ComplementIterOwned<I, C> = ComplementIter<IntervalIterOwned<I, C>, I, C>;

/// A trait for interval containers that generates an iterator over the
/// complement of the intervals in the container.
///
/// ```text
/// (s)   x----y  x------y   x-----y   x-----y
/// ===============================================
/// (c)        y--x      y---x     y---x
/// ```
///
/// # Examples
///
/// ```
/// use bedrs::{BaseInterval, Coordinates, IntervalContainer};
///
/// let intervals = vec![
///     BaseInterval::new(10, 20),
///     BaseInterval::new(30, 40),
///     BaseInterval::new(50, 60),
///     BaseInterval::new(70, 80),
/// ];
///
/// let expected = vec![
///     BaseInterval::new(20, 30),
///     BaseInterval::new(40, 50),
///     BaseInterval::new(60, 70),
/// ];
///
/// let set = IntervalContainer::from_unsorted(intervals);
/// let comp_iter = set.complement().unwrap();
/// let complements: Vec<_> = comp_iter.collect();
///
/// assert_eq!(complements.len(), expected.len());
/// for (obs, exp) in complements.iter().zip(expected.iter()) {
///    assert!(obs.eq(exp));
/// }
/// ```
impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn complement(self) -> Result<ComplementIterOwned<I, C>> {
        if self.is_sorted() {
            Ok(self.complement_unchecked())
        } else {
            bail!(SetError::UnsortedSet)
        }
    }

    pub fn complement_unchecked(self) -> ComplementIterOwned<I, C> {
        ComplementIter::new(self.into_iter())
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::{ChromBounds, IntervalBounds},
        BaseInterval, Bed3, IntervalContainer,
    };

    fn validate_records<I, C>(obs: &[I], exp: &[I])
    where
        I: IntervalBounds<C>,
        C: ChromBounds,
    {
        assert_eq!(obs.len(), exp.len());
        for (obs, exp) in obs.iter().zip(exp.iter()) {
            assert!(obs.eq(exp));
        }
    }

    #[test]
    fn complement_unsorted() {
        let intervals = vec![BaseInterval::new(10, 20), BaseInterval::new(30, 40)];
        let set = IntervalContainer::from_iter(intervals);
        assert!(set.complement().is_err());
    }

    #[test]
    fn complement_unmerged_boundary() {
        let intervals = vec![BaseInterval::new(10, 20), BaseInterval::new(20, 30)];
        let set = IntervalContainer::from_iter(intervals);
        let comp_iter = set.complement();
        assert!(comp_iter.is_err());
    }

    #[test]
    fn complement_unmerged_overlapping() {
        let intervals = vec![BaseInterval::new(10, 20), BaseInterval::new(18, 30)];
        let set = IntervalContainer::from_iter(intervals);
        let comp_iter = set.complement();
        assert!(comp_iter.is_err());
    }

    #[test]
    /// x---------------y    i--------j
    /// ================================
    ///                 y----i
    fn complement_a() {
        let intervals = vec![BaseInterval::new(10, 20), BaseInterval::new(30, 40)];
        let expected = vec![BaseInterval::new(20, 30)];
        let set = IntervalContainer::from_unsorted(intervals);
        let comp_iter = set.complement().unwrap();
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// x---------------y    i--------j k----l
    /// =========================================
    ///                 y----i        j-k
    fn complement_b() {
        let intervals = vec![
            BaseInterval::new(10, 20),
            BaseInterval::new(30, 40),
            BaseInterval::new(50, 60),
        ];
        let expected = vec![BaseInterval::new(20, 30), BaseInterval::new(40, 50)];
        let set = IntervalContainer::from_unsorted(intervals);
        let comp_iter = set.complement().unwrap();
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// |1| x---------------y    i--------j |2| k----l
    /// ===============================================
    /// |1|                 y----i
    fn complement_c() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let expected = vec![Bed3::new(1, 20, 30)];
        let set = IntervalContainer::from_unsorted(intervals);
        let comp_iter = set.complement().unwrap();
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// |1| x---------------y    i--------j |2| k----l  m----n
    /// =======================================================
    /// |1|                 y----i          |2|      l--m
    fn complement_d() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(2, 10, 20),
            Bed3::new(2, 30, 40),
        ];
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(2, 20, 30)];
        let set = IntervalContainer::from_unsorted(intervals);
        let comp_iter = set.complement().unwrap();
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }
}
