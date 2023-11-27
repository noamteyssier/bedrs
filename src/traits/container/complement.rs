use anyhow::{bail, Result};

use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    types::{iterator::ComplementIter, IntervalIterOwned},
    Container,
};

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
/// use bedrs::{Interval, Complement, Container, Coordinates, IntervalContainer};
///
/// let intervals = vec![
///     Interval::new(10, 20),
///     Interval::new(30, 40),
///     Interval::new(50, 60),
///     Interval::new(70, 80),
/// ];
///
/// let expected = vec![
///     Interval::new(20, 30),
///     Interval::new(40, 50),
///     Interval::new(60, 70),
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
pub trait Complement<C, T, I>: Container<C, T, I>
where
    C: ChromBounds,
    T: ValueBounds,
    I: IntervalBounds<C, T>,
{
    fn complement(self) -> Result<ComplementIter<IntervalIterOwned<I, C, T>, I, C, T>> {
        if self.is_sorted() {
            Ok(self.complement_unchecked())
        } else {
            bail!(SetError::UnsortedSet)
        }
    }

    fn complement_unchecked(self) -> ComplementIter<IntervalIterOwned<I, C, T>, I, C, T> {
        ComplementIter::new(self.into_iter())
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{
        traits::{ChromBounds, IntervalBounds, ValueBounds},
        GenomicInterval, Interval, IntervalContainer,
    };

    fn validate_records<I, C, T>(obs: &[I], exp: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ChromBounds,
        T: ValueBounds,
    {
        assert_eq!(obs.len(), exp.len());
        for (obs, exp) in obs.iter().zip(exp.iter()) {
            assert!(obs.eq(exp));
        }
    }

    #[test]
    fn complement_unsorted() {
        let intervals = vec![Interval::new(10, 20), Interval::new(30, 40)];
        let set = IntervalContainer::from_iter(intervals);
        assert!(set.complement().is_err());
    }

    #[test]
    #[should_panic]
    fn complement_unmerged_boundary() {
        let intervals = vec![Interval::new(10, 20), Interval::new(20, 30)];
        let set = IntervalContainer::from_iter(intervals);
        let comp_iter = set.complement().unwrap();
        let _complements: Vec<_> = comp_iter.collect();
    }

    #[test]
    #[should_panic]
    fn complement_unmerged_overlapping() {
        let intervals = vec![Interval::new(10, 20), Interval::new(18, 30)];
        let set = IntervalContainer::from_iter(intervals);
        let comp_iter = set.complement().unwrap();
        let _complements: Vec<_> = comp_iter.collect();
    }

    #[test]
    /// x---------------y    i--------j
    /// ================================
    ///                 y----i
    fn complement_a() {
        let intervals = vec![Interval::new(10, 20), Interval::new(30, 40)];
        let expected = vec![Interval::new(20, 30)];
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
            Interval::new(10, 20),
            Interval::new(30, 40),
            Interval::new(50, 60),
        ];
        let expected = vec![Interval::new(20, 30), Interval::new(40, 50)];
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
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(2, 50, 60),
        ];
        let expected = vec![GenomicInterval::new(1, 20, 30)];
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
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(2, 10, 20),
            GenomicInterval::new(2, 30, 40),
        ];
        let expected = vec![
            GenomicInterval::new(1, 20, 30),
            GenomicInterval::new(2, 20, 30),
        ];
        let set = IntervalContainer::from_unsorted(intervals);
        let comp_iter = set.complement().unwrap();
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }
}
