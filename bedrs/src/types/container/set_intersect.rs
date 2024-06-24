use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::Query,
    Intersect, IntervalContainer,
};

impl<'a, I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `other`
    ///
    /// # Panics
    /// Panics if the intersection of two intervals fails to be computed (like from unsorted
    /// containers)
    #[must_use]
    pub fn ix_set_target<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C>,
        method: Query,
    ) -> Box<dyn Iterator<Item = Iv> + 'a>
    where
        Iv: IntervalBounds<C> + 'a,
        &'a Iv: Intersect<C>,
    {
        let ix_iter = self.iter().flat_map(move |iv| {
            let overlaps = other
                .query_iter(iv, method)
                .expect("Failed to find overlaps with provided query method")
                .cloned();
            overlaps.into_iter().map(|ov| match iv.intersect(&ov) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            })
        });
        Box::new(ix_iter)
    }

    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `self`
    ///
    /// # Panics
    /// Panics if the intersection of two intervals fails to be computed (like from unsorted
    /// containers)
    #[must_use]
    pub fn ix_set_query<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C>,
        method: Query,
    ) -> Box<dyn Iterator<Item = I> + 'a>
    where
        Iv: IntervalBounds<C> + 'a,
    {
        let ix_iter = self.iter().flat_map(move |iv| {
            let overlaps = other
                .query_iter(iv, method)
                .expect("Failed to find overlaps with provided query method");
            overlaps.into_iter().map(|ov| match ov.intersect(iv) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            })
        });
        Box::new(ix_iter)
    }
}
