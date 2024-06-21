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
    pub fn internal(&self, name: &C) -> Result<SubtractFromIter<I, I, C>> {
        let Some(subtree) = self.subtree(name) else {
            bail!(SetError::MissingSubtreeName)
        };
        if self.is_sorted() {
            let span = subtree.span()?;
            Ok(SubtractFromIter::new(self, &span))
        } else {
            bail!(SetError::UnsortedSet)
        }
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
        assert!(set.internal(&0).is_err());
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
        let internal_set: IntervalContainer<_, _> = set.internal(&0).unwrap().collect();
        assert_eq!(internal_set.len(), 1);
        let subtree = internal_set.subtree(&0).unwrap();
        assert_eq!(subtree[0].start(), 3);
        assert_eq!(subtree[0].end(), 6);
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
        let internal_set: IntervalContainer<_, _> = set.internal(&0).unwrap().collect();
        assert_eq!(internal_set.len(), 2);
        let subtree = internal_set.subtree(&0).unwrap();
        assert_eq!(subtree[0].start(), 3);
        assert_eq!(subtree[0].end(), 6);
        assert_eq!(subtree[1].start(), 10);
        assert_eq!(subtree[1].end(), 12);
    }
}
