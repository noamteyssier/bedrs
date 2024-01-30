use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::QueryMethod,
    Intersect, IntervalContainer,
};

impl<'a, I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `other`
    pub fn ix_set_target<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C, T>,
        query_method: QueryMethod<T>,
    ) -> Box<dyn Iterator<Item = Iv> + 'a>
    where
        Iv: IntervalBounds<C, T> + 'a,
    {
        let ix_iter = self.records().iter().flat_map(move |iv| {
            let overlaps = other
                .find_method(iv, query_method)
                .expect("Failed to find overlaps with provided query method");
            let intersections = overlaps.into_iter().map(|ov| match iv.intersect(&ov) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            });
            intersections
        });
        Box::new(ix_iter)
    }

    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `self`
    pub fn ix_set_query<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C, T>,
        query_method: QueryMethod<T>,
    ) -> Box<dyn Iterator<Item = I> + 'a>
    where
        Iv: IntervalBounds<C, T> + 'a,
    {
        let ix_iter = self.records().iter().flat_map(move |iv| {
            let overlaps = other
                .find_method(iv, query_method)
                .expect("Failed to find overlaps with provided query method");
            let intersections = overlaps.into_iter().map(|ov| match ov.intersect(iv) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            });
            intersections
        });
        Box::new(ix_iter)
    }
}
