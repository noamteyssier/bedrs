use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::SubtractFromIter,
    Container,
};
use anyhow::{bail, Result};

pub trait Internal<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    /// Returns all non-overlapping intervals of the interval
    /// set within its span
    ///
    /// ```text
    /// (a)     i----j
    /// (b)              k----l
    /// (c)                      m-----n
    /// (span)  i----------------------n
    /// ================================
    /// (i)          j---k
    /// (ii)                  l--m
    /// ```
    fn internal<'a>(&'a self) -> Result<SubtractFromIter<T, I>> {
        if self.is_sorted() {
            let span = self.span()?;
            Ok(self.internal_unchecked(span))
        } else {
            bail!("Interval set is not sorted")
        }
    }

    // Unchecked version of [internal](Self::internal).
    //
    // Does not check if the interval set is sorted.
    // Span must still be valid.
    fn internal_unchecked<'a>(&'a self, span: I) -> SubtractFromIter<T, I> {
        SubtractFromIter::new(self, &span)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{Coordinates, Interval, IntervalSet};

    #[test]
    fn internal_unsorted() {
        let set = IntervalSet::new(vec![
            Interval::new(1, 5),
            Interval::new(2, 4),
            Interval::new(3, 6),
        ]);
        assert!(set.internal().is_err());
    }

    #[test]
    /// (a)  i---j
    /// (b)        k---l
    /// ==================
    /// (i)      j-k
    fn internal_a() {
        let set =
            IntervalSet::from_sorted(vec![Interval::new(1, 3), Interval::new(6, 10)]).unwrap();
        let span = set.span().unwrap();
        let sub = set.internal_unchecked(span);
        let internal_set = IntervalSet::from_iter(sub);
        assert_eq!(internal_set.len(), 1);
        assert_eq!(internal_set.records()[0].start(), 3);
        assert_eq!(internal_set.records()[0].end(), 6);
    }

    #[test]
    /// (a)  i---j
    /// (b)        k---l
    /// (c)                m-----n
    /// ============================
    /// (i)      j-k
    /// (ii)           l---m
    fn internal_b() {
        let set = IntervalSet::from_sorted(vec![
            Interval::new(1, 3),
            Interval::new(6, 10),
            Interval::new(12, 15),
        ])
        .unwrap();
        let span = set.span().unwrap();
        let sub = set.internal_unchecked(span);
        let internal_set = IntervalSet::from_iter(sub);
        assert_eq!(internal_set.len(), 2);
        assert_eq!(internal_set.records()[0].start(), 3);
        assert_eq!(internal_set.records()[0].end(), 6);
        assert_eq!(internal_set.records()[1].start(), 10);
        assert_eq!(internal_set.records()[1].end(), 12);
    }
}
