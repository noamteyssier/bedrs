// use super::Container;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    types::{FindIterSorted, FindIterSortedOwned, IntervalContainer, Query},
};
use anyhow::Result;

/// A trait to query set overlaps through a container
impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Find all intervals that overlap a query interval
    /// and return an iterator over the intervals.
    ///
    /// Will return an error if the set is not sorted.
    pub fn query_iter<'a, Iv>(
        &'a self,
        query: &'a Iv,
        method: Query<T>,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if self.is_sorted() {
            self.query_iter_unchecked(query, method)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Find all intervals that overlap a query interval
    /// and return an iterator over the intervals.
    ///
    /// Assumes the set is sorted.
    pub fn query_iter_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        method: Query<T>,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        method.validate()?;
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            method,
        ))
    }
    /// Find all intervals that overlap a query interval
    /// and return an iterator over the intervals.
    ///
    /// Will return an error if the set is not sorted.
    pub fn query_iter_owned<Iv>(
        &self,
        query: Iv,
        method: Query<T>,
    ) -> Result<FindIterSortedOwned<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if self.is_sorted() {
            self.query_iter_owned_unchecked(query, method)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Find all intervals that overlap a query interval
    /// and return an iterator over the intervals.
    ///
    /// Assumes the set is sorted.
    pub fn query_iter_owned_unchecked<Iv>(
        &self,
        query: Iv,
        method: Query<T>,
    ) -> Result<FindIterSortedOwned<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        method.validate()?;
        let bound = self.lower_bound_unchecked(&query);
        Ok(FindIterSortedOwned::new(
            self.records(),
            query,
            bound,
            method,
        ))
    }

    /// Find all intervals that overlap a query interval
    /// and return an `IntervalContainer` containing the intervals.
    ///
    /// Will return an error if the set is not sorted.
    pub fn query_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        method: Query<T>,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        Ok(self.query_iter_unchecked(query, method)?.cloned().collect())
    }

    /// Find all intervals that overlap a query interval
    /// and return an `IntervalContainer` containing the intervals.
    ///
    /// Assumes the set is sorted.
    pub fn query<'a, Iv>(
        &'a self,
        query: &'a Iv,
        method: Query<T>,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        self.query_iter(query, method)
            .map(|iter| iter.cloned().collect())
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod testing {
    use crate::{
        traits::{ChromBounds, IntervalBounds, ValueBounds},
        types::{Query, QueryMethod},
        BaseInterval, Bed3, Coordinates, IntervalContainer,
    };

    fn validate_set<C, I, T>(set: &IntervalContainer<I, C, T>, expected: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ChromBounds,
        T: ValueBounds,
    {
        for idx in 0..expected.len() {
            let c1 = &set.records()[idx];
            let c2 = &expected[idx];
            assert!(c1.eq(c2));
        }
    }

    fn validate_iter<I, C, T>(iter: impl Iterator<Item = I>, expected: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ChromBounds,
        T: ValueBounds,
    {
        let observed = iter.collect::<Vec<I>>();
        for idx in 0..expected.len() {
            let c1 = &observed[idx];
            let c2 = &expected[idx];
            assert!(c1.eq(c2));
        }
    }

    #[test]
    fn find() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let method = Query::default();
        let overlaps = set.query(&query, method).unwrap();
        assert_eq!(overlaps.len(), 4);
    }

    #[test]
    fn find_containing() {
        let query = BaseInterval::new(0, 100);
        let intervals = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set.query(&query, Query::default()).unwrap();
        assert_eq!(overlaps.len(), 1);
    }

    #[test]
    fn find_containing_iter_sorted() {
        let query = BaseInterval::new(0, 100);
        let intervals = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set.query_iter(&query, Query::default()).unwrap();
        let counts = overlaps.count();
        assert_eq!(counts, 1);
    }

    #[test]
    fn find_minimum_overlap() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let method = Query::new_predicate(QueryMethod::CompareBy(5));
        let overlaps = set.query(&query, method).unwrap();
        assert_eq!(overlaps.len(), 3);
    }

    #[test]
    fn find_exact() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let method = Query::new_predicate(QueryMethod::CompareExact(7));
        let overlaps = set.query(&query, method).unwrap();
        assert_eq!(overlaps.len(), 1);
    }

    #[test]
    fn find_iter() {
        let query = BaseInterval::new(5, 12);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let num_overlaps = set.query_iter(&query, Query::default()).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_owned() {
        let query = BaseInterval::new(5, 12);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let num_overlaps = set.query_iter(&query, Query::default()).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted() {
        let query = BaseInterval::new(5, 12);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let num_overlaps = set.query_iter(&query, Query::default()).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_owned() {
        let query = BaseInterval::new(5, 12);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let num_overlaps = set.query_iter(&query, Query::default()).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_wrong_order() {
        let query = BaseInterval::new(5, 12);
        let starts = [15, 20, 25, 10];
        let ends = [45, 50, 55, 40];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_iter(records);
        let overlaps = set.query_iter(&query, Query::default());
        assert!(overlaps.is_err());
    }

    #[test]
    fn find_iter_min() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareBy(5)))
            .unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }

    #[test]
    fn find_iter_exact() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareExact(7)))
            .unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_min() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareBy(5)))
            .unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }

    #[test]
    fn find_iter_sorted_exact() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareExact(7)))
            .unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_min_genomic() {
        let query = Bed3::new(3, 17, 27);
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 15, 25),
            Bed3::new(3, 10, 20), // bounded, but missing overlap req
            Bed3::new(3, 15, 25), // first
            Bed3::new(3, 20, 30), // last
            Bed3::new(3, 40, 50), // unbounded
            Bed3::new(4, 10, 20),
            Bed3::new(4, 25, 35),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let mut overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareBy(5)))
            .unwrap()
            .copied();
        let first = overlaps.next().unwrap();
        let last = overlaps.last().unwrap();
        assert!(first.eq(&Bed3::new(3, 15, 25)));
        assert!(last.eq(&Bed3::new(3, 20, 30)));
    }

    #[test]
    fn find_iter_sorted_exact_genomic() {
        let query = Bed3::new(3, 17, 27);
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 15, 25),
            Bed3::new(3, 10, 20), // bounded, but missing overlap req
            Bed3::new(3, 15, 25), // bounded, but missing overlap req
            Bed3::new(3, 20, 30), // first and last
            Bed3::new(3, 40, 50), // unbounded
            Bed3::new(4, 10, 20),
            Bed3::new(4, 25, 35),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let mut overlaps = set
            .query_iter(&query, Query::new_predicate(QueryMethod::CompareExact(7)))
            .unwrap()
            .copied();
        let first = overlaps.next().unwrap();
        let last = overlaps.last();
        assert!(first.eq(&Bed3::new(3, 20, 30)));
        assert!(last.is_none());
    }

    #[test]
    fn find_query_frac_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            BaseInterval::new(0, 10),
            BaseInterval::new(5, 15), // first
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
            BaseInterval::new(17, 27), // bounded, but missing overlap req
            BaseInterval::new(20, 30),
        ];
        let expected = vec![
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByQueryFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_query_frac_b() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.2;
        let intervals = vec![
            BaseInterval::new(0, 10),
            BaseInterval::new(5, 15), // first
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
            BaseInterval::new(17, 27), // last
            BaseInterval::new(20, 30),
        ];
        let expected = vec![
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
            BaseInterval::new(17, 27),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByQueryFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_query_frac_c() {
        let query = BaseInterval::new(10, 20);
        let frac = 1.0;
        let intervals = vec![
            BaseInterval::new(0, 10),
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20), // only
            BaseInterval::new(15, 25),
            BaseInterval::new(17, 27),
            BaseInterval::new(20, 30),
        ];
        let expected = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByQueryFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_iter_sorted_query_frac() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            BaseInterval::new(0, 10),
            BaseInterval::new(5, 15), // first
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
            BaseInterval::new(17, 27), // bounded, but missing overlap req
            BaseInterval::new(20, 30),
        ];
        let expected = vec![
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
            BaseInterval::new(15, 25),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlap_iter = set
            .query_iter(
                &query,
                Query::new_predicate(QueryMethod::CompareByQueryFraction(frac)),
            )
            .unwrap()
            .copied();

        validate_iter(overlap_iter, &expected);
    }

    #[test]
    fn find_target_frac_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            BaseInterval::new(2, 12), // bounded, but missing overlap req
            BaseInterval::new(5, 15), // first
            BaseInterval::new(7, 17),
            BaseInterval::new(7, 37),  // bounded, but missing overlap req
            BaseInterval::new(10, 20), // last
            BaseInterval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByTargetFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_target_frac_b() {
        let query = BaseInterval::new(10, 20);
        let frac = 1.0;
        let intervals = vec![
            BaseInterval::new(2, 12),  // bounded, but missing overlap req
            BaseInterval::new(5, 15),  // bounded, but missing overlap req
            BaseInterval::new(7, 17),  // bounded, but missing overlap req
            BaseInterval::new(7, 37),  // bounded, but missing overlap req
            BaseInterval::new(10, 20), // only
            BaseInterval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByTargetFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_target_frac_c() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            BaseInterval::new(8, 18), // bounded, but missing overlap req
            BaseInterval::new(9, 19), // first
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21), // last
            BaseInterval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            BaseInterval::new(9, 19),
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareByTargetFraction(frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_iter_sorted_target_frac() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            BaseInterval::new(2, 12), // bounded, but missing overlap req
            BaseInterval::new(5, 15), // first
            BaseInterval::new(7, 17),
            BaseInterval::new(7, 37),  // bounded, but missing overlap req
            BaseInterval::new(10, 20), // last
            BaseInterval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            BaseInterval::new(5, 15),
            BaseInterval::new(7, 17),
            BaseInterval::new(10, 20),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlap_iter = set
            .query_iter(
                &query,
                Query::new_predicate(QueryMethod::CompareByTargetFraction(frac)),
            )
            .unwrap()
            .copied();
        validate_iter(overlap_iter, &expected);
    }

    #[test]
    fn find_reciprocal_frac_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            BaseInterval::new(8, 18),
            // overlaps by 90% of target and query
            BaseInterval::new(9, 19), // only
            // overlaps by 90% of query but not target
            BaseInterval::new(9, 20),
            // overlaps by >90% of target but not query
            BaseInterval::new(15, 18),
            // outside interval
            BaseInterval::new(20, 30),
        ];
        let expected = vec![BaseInterval::new(9, 19)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionAnd(frac, frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_iter_sorted_reciprocal_frac_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            BaseInterval::new(8, 18),
            // overlaps by 90% of target and query
            BaseInterval::new(9, 19), // only
            // overlaps by 90% of query but not target
            BaseInterval::new(9, 20),
            // overlaps by >90% of target but not query
            BaseInterval::new(15, 18),
            // outside interval
            BaseInterval::new(20, 30),
        ];
        let expected = vec![BaseInterval::new(9, 19)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlap_iter = set
            .query_iter(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionAnd(frac, frac)),
            )
            .unwrap()
            .copied();
        validate_iter(overlap_iter, &expected);
    }

    #[test]
    fn find_reciprocal_frac_uneven() {
        let query = BaseInterval::new(10, 20);
        let f_query = 0.9;
        let f_target = 0.8;
        let intervals = vec![
            BaseInterval::new(7, 17), // bounded, but missing overlap req
            BaseInterval::new(8, 18), // bounded, but missing overlap req on query
            BaseInterval::new(9, 19), // first
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21), // last
            BaseInterval::new(12, 22), // bounded, but missing overlap req on query
            BaseInterval::new(13, 23), // bounded, but missing overlap req
        ];
        let expected = vec![
            BaseInterval::new(9, 19),
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionAnd(f_query, f_target)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_reciprocal_frac_either_uneven() {
        let query = BaseInterval::new(10, 20);
        let f_query = 0.9;
        let f_target = 0.8;
        let intervals = vec![
            BaseInterval::new(7, 17), // bounded, but missing overlap req
            BaseInterval::new(8, 18), // first
            BaseInterval::new(9, 19),
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21),
            BaseInterval::new(12, 22), // last
            BaseInterval::new(13, 23), // bounded, but missing overlap req
        ];
        let expected = vec![
            BaseInterval::new(8, 18),
            BaseInterval::new(9, 19),
            BaseInterval::new(10, 20),
            BaseInterval::new(11, 21),
            BaseInterval::new(12, 22),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionOr(f_query, f_target)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_reciprocal_frac_either_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            BaseInterval::new(8, 18),
            // overlaps by 90% of target and query
            BaseInterval::new(9, 19), // first
            // overlaps by 90% of query but not target
            BaseInterval::new(9, 20),
            // overlaps by >90% of target but not query
            BaseInterval::new(15, 18), // last
            // outside interval
            BaseInterval::new(20, 30),
        ];
        let expected = vec![
            BaseInterval::new(9, 19),
            BaseInterval::new(9, 20),
            BaseInterval::new(15, 18),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set
            .query(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionOr(frac, frac)),
            )
            .unwrap();
        validate_set(&overlaps, &expected);
    }

    #[test]
    fn find_iter_sorted_reciprocal_frac_either_a() {
        let query = BaseInterval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            BaseInterval::new(8, 18),
            // overlaps by 90% of target and query
            BaseInterval::new(9, 19), // first
            // overlaps by 90% of query but not target
            BaseInterval::new(9, 20),
            // overlaps by >90% of target but not query
            BaseInterval::new(15, 18), // last
            // outside interval
            BaseInterval::new(20, 30),
        ];
        let expected = vec![
            BaseInterval::new(9, 19),
            BaseInterval::new(9, 20),
            BaseInterval::new(15, 18),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlap_iter = set
            .query_iter(
                &query,
                Query::new_predicate(QueryMethod::CompareReciprocalFractionOr(frac, frac)),
            )
            .unwrap()
            .copied();
        validate_iter(overlap_iter, &expected);
    }

    #[test]
    fn find_query_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        let method = Query::new_predicate(QueryMethod::CompareByQueryFraction(0.0));
        assert!(set.query(&query, method).is_err());
        let method = Query::new_predicate(QueryMethod::CompareByQueryFraction(1.01));
        assert!(set.query(&query, method).is_err());
    }

    #[test]
    fn find_target_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        let method = Query::new_predicate(QueryMethod::CompareByTargetFraction(0.0));
        assert!(set.query(&query, method).is_err());
        let method = Query::new_predicate(QueryMethod::CompareByTargetFraction(1.01));
        assert!(set.query(&query, method).is_err());
    }

    #[test]
    fn find_reciprocal_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        let method = Query::new_predicate(QueryMethod::CompareReciprocalFractionAnd(0.0, 0.0));
        assert!(set.query(&query, method).is_err());
        let method = Query::new_predicate(QueryMethod::CompareReciprocalFractionAnd(1.01, 1.01));
        assert!(set.query(&query, method).is_err());
    }

    #[test]
    fn find_reciprocal_frac_either_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        let method = Query::new_predicate(QueryMethod::CompareReciprocalFractionOr(0.0, 0.0));
        assert!(set.query(&query, method).is_err());
        let method = Query::new_predicate(QueryMethod::CompareReciprocalFractionOr(1.01, 1.01));
        assert!(set.query(&query, method).is_err());
    }
}
