use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::iterator::QueryMethod,
    Container, Find, Intersect,
};

pub trait SetIntersect<'a, C, T, I>: Container<C, T, I>
where
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
    I: IntervalBounds<C, T> + 'a,
{
    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `other`
    fn ix_set_target<Co, Iv>(
        &'a self,
        other: &'a Co,
        query_method: QueryMethod<T>,
    ) -> Box<dyn Iterator<Item = Iv> + 'a>
    where
        Co: Container<C, T, Iv> + 'a,
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
    fn ix_set_query<Co, Iv>(
        &'a self,
        other: &'a Co,
        query_method: QueryMethod<T>,
    ) -> Box<dyn Iterator<Item = I> + 'a>
    where
        Co: Container<C, T, Iv> + 'a,
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
