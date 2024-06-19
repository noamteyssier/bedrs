use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds},
    types::SubtractFromIter,
    IntervalContainer,
};
use anyhow::{bail, Result};

/// Identifies al non-overlapping intervals within the span of the interval set
impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
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
    pub fn internal(&self) -> Result<SubtractFromIter<I, I, C>> {
        if self.is_sorted() {
            let span = self.span()?;
            Ok(self.internal_unchecked(&span))
        } else {
            bail!(SetError::UnsortedSet)
        }
    }

    // Unchecked version of [internal](Self::internal).
    //
    // Does not check if the interval set is sorted.
    // Span must still be valid.
    pub fn internal_unchecked(&self, span: &I) -> SubtractFromIter<I, I, C> {
        SubtractFromIter::new(self, span)
    }
}

#[cfg(test)]
mod testing {
    use crate::{BaseInterval, Coordinates, IntervalContainer};

    #[test]
    fn internal_unsorted() {
        let set = IntervalContainer::new(vec![
            BaseInterval::new(1, 5),
            BaseInterval::new(2, 4),
            BaseInterval::new(3, 6),
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
            IntervalContainer::from_sorted(vec![BaseInterval::new(1, 3), BaseInterval::new(6, 10)])
                .unwrap();
        let span = set.span().unwrap();
        let internal_set: IntervalContainer<_, _> = set.internal_unchecked(&span).collect();
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
        let set = IntervalContainer::from_sorted(vec![
            BaseInterval::new(1, 3),
            BaseInterval::new(6, 10),
            BaseInterval::new(12, 15),
        ])
        .unwrap();
        let span = set.span().unwrap();
        let internal_set: IntervalContainer<_, _> = set.internal_unchecked(&span).collect();
        assert_eq!(internal_set.len(), 2);
        assert_eq!(internal_set.records()[0].start(), 3);
        assert_eq!(internal_set.records()[0].end(), 6);
        assert_eq!(internal_set.records()[1].start(), 10);
        assert_eq!(internal_set.records()[1].end(), 12);
    }
}
