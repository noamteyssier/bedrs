use super::Container;
use crate::{
    traits::{errors::SetError, IntervalBounds, ValueBounds},
    types::{iterator::QueryMethod, FindIter, FindIterSorted},
    Bound,
};

/// A trait to query set overlaps through a container
pub trait Find<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    type ContainerType: Container<T, I>;

    /// Finds all intervals that overlap a query and returns
    /// the same `Container` type with all found regions.
    fn find(&self, query: &I) -> Self::ContainerType {
        let records = self
            .find_iter(query)
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<I>>();
        Self::ContainerType::new(records)
    }

    /// Finds all intervals that overlap a query by some minimum
    /// amount and returns the same `Container` type with all found regions.
    fn find_min(&self, query: &I, minimum: T) -> Self::ContainerType {
        let records = self
            .find_iter_min(query, minimum)
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<I>>();
        Self::ContainerType::new(records)
    }

    /// Finds all intervals that overlap a query by some exact
    /// amount and returns the same `Container` type with all found regions.
    fn find_exact(&self, query: &I, exact: T) -> Self::ContainerType {
        let records = self
            .find_iter_exact(query, exact)
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<I>>();
        Self::ContainerType::new(records)
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of the query length and returns the same `Container` type with all found regions.
    fn find_query_frac(&self, query: &I, frac: f64) -> Result<Self::ContainerType, SetError> {
        let records = match self.find_iter_query_frac(query, frac) {
            Ok(iter) => iter.into_iter().map(|x| x.to_owned()).collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(Self::ContainerType::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of the target length and returns the same `Container` type with all found regions.
    fn find_target_frac(&self, query: &I, frac: f64) -> Result<Self::ContainerType, SetError> {
        let records = match self.find_iter_target_frac(query, frac) {
            Ok(iter) => iter.into_iter().map(|x| x.to_owned()).collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(Self::ContainerType::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of **both** the query and target lengths and returns the
    /// same `Container` type with all found regions.
    fn find_reciprocal_frac(&self, query: &I, frac: f64) -> Result<Self::ContainerType, SetError> {
        let records = match self.find_iter_reciprocal_frac(query, frac) {
            Ok(iter) => iter.into_iter().map(|x| x.to_owned()).collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(Self::ContainerType::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of **either** the query and target lengths and returns the
    /// same `Container` type with all found regions.
    fn find_reciprocal_frac_either(
        &self,
        query: &I,
        frac: f64,
    ) -> Result<Self::ContainerType, SetError> {
        let records = match self.find_iter_reciprocal_frac_either(query, frac) {
            Ok(iter) => iter.into_iter().map(|x| x.to_owned()).collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(Self::ContainerType::new(records))
    }

    /// Creates an iterator that finds all overlapping regions
    ///
    /// Does not assume a sorted Container
    fn find_iter<'a>(&'a self, query: &'a I) -> FindIter<'_, T, I> {
        FindIter::new(self.records(), query, QueryMethod::Compare)
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some minimum overlap
    ///
    /// Does not assume a sorted Container
    fn find_iter_min<'a>(&'a self, query: &'a I, minimum: T) -> FindIter<'_, T, I> {
        FindIter::new(self.records(), query, QueryMethod::CompareBy(minimum))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some exact overlap
    ///
    /// Does not assume a sorted Container
    fn find_iter_exact<'a>(&'a self, query: &'a I, exact: T) -> FindIter<'_, T, I> {
        FindIter::new(self.records(), query, QueryMethod::CompareExact(exact))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of the query length
    ///
    /// Does not assume a sorted Container
    fn find_iter_query_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIter<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareByQueryFraction(frac),
        ))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of the target length
    ///
    /// Does not assume a sorted Container
    fn find_iter_target_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIter<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareByTargetFraction(frac),
        ))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of **both** the query and target length
    ///
    /// Does not assume a sorted Container
    fn find_iter_reciprocal_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIter<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareReciprocalFractionAnd(frac),
        ))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of **either** the query and target length
    ///
    /// Does not assume a sorted Container
    fn find_iter_reciprocal_frac_either<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIter<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareReciprocalFractionOr(frac),
        ))
    }

    /// Creates a Result Iterator that finds all overlapping regions
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted<'a>(&'a self, query: &'a I) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some minimum overlap
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_min<'a>(
        &'a self,
        query: &'a I,
        minimum: T,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_min_unchecked(query, minimum))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some exact overlap
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_exact<'a>(
        &'a self,
        query: &'a I,
        exact: T,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_exact_unchecked(query, exact))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some fraction of the query length
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_query_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_query_frac_unchecked(query, frac)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some fraction of the target length
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_target_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_target_frac_unchecked(query, frac)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some fraction of **both** the query and target length
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_reciprocal_frac<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_reciprocal_frac_unchecked(query, frac)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some fraction of **both** the query and target length
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted_reciprocal_frac_either<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_reciprocal_frac_either_unchecked(query, frac)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates an Iterator that finds all overlapping regions
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_unchecked<'a>(&'a self, query: &'a I) -> FindIterSorted<'_, T, I> {
        FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::Compare,
        )
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some minimum overlap
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_min_unchecked<'a>(
        &'a self,
        query: &'a I,
        minimum: T,
    ) -> FindIterSorted<'_, T, I> {
        FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareBy(minimum),
        )
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some exact overlap
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_exact_unchecked<'a>(
        &'a self,
        query: &'a I,
        exact: T,
    ) -> FindIterSorted<'_, T, I> {
        FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareExact(exact),
        )
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some fraction of the query length
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_query_frac_unchecked<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareByQueryFraction(frac),
        ))
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some fraction of the target length
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_target_frac_unchecked<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareByTargetFraction(frac),
        ))
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some fraction of **both** the query and target length
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_reciprocal_frac_unchecked<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareReciprocalFractionAnd(frac),
        ))
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some fraction of **either** the query and target length
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_reciprocal_frac_either_unchecked<'a>(
        &'a self,
        query: &'a I,
        frac: f64,
    ) -> Result<FindIterSorted<'_, T, I>, SetError> {
        if frac <= 0.0 || frac > 1.0 {
            return Err(SetError::FractionUnbounded { frac });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareReciprocalFractionOr(frac),
        ))
    }
}

#[cfg(test)]
mod testing {
    use super::Find;
    use crate::{
        traits::Container, Coordinates, GenomicInterval, GenomicIntervalSet, Interval, IntervalSet,
    };

    #[test]
    fn find() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find(&query);
        assert_eq!(overlaps.len(), 4);
    }

    #[test]
    fn find_minimum() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_min(&query, 5);
        assert_eq!(overlaps.len(), 3);
    }

    #[test]
    fn find_exact() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_exact(&query, 7);
        assert_eq!(overlaps.len(), 1);
    }

    #[test]
    fn find_iter() {
        let query = Interval::new(5, 12);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let num_overlaps = set.find_iter(&query).count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted() {
        let query = Interval::new(5, 12);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let num_overlaps = set.find_iter_sorted(&query).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_wrong_order() {
        let query = Interval::new(5, 12);
        let starts = vec![15, 20, 25, 10];
        let ends = vec![45, 50, 55, 40];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_iter_sorted(&query);
        assert!(overlaps.is_err());
    }

    #[test]
    fn find_iter_min() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_iter_min(&query, 5);
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }

    #[test]
    fn find_iter_exact() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_iter_exact(&query, 7);
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_min() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let overlaps = set.find_iter_sorted_min(&query, 5).unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }

    #[test]
    fn find_iter_sorted_exact() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let overlaps = set.find_iter_sorted_exact(&query, 7).unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn find_iter_sorted_min_genomic() {
        let query = GenomicInterval::new(3, 17, 27);
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 15, 25),
            GenomicInterval::new(3, 10, 20), // bounded, but missing overlap req
            GenomicInterval::new(3, 15, 25), // first
            GenomicInterval::new(3, 20, 30), // last
            GenomicInterval::new(3, 40, 50), // unbounded
            GenomicInterval::new(4, 10, 20),
            GenomicInterval::new(4, 25, 35),
        ];
        let set = GenomicIntervalSet::from_sorted(intervals).unwrap();
        let mut overlaps = set
            .find_iter_sorted_min(&query, 5)
            .unwrap()
            .into_iter()
            .cloned();
        let first = overlaps.next().unwrap();
        let last = overlaps.last().unwrap();
        assert_eq!(first, GenomicInterval::new(3, 15, 25));
        assert_eq!(last, GenomicInterval::new(3, 20, 30));
    }

    #[test]
    fn find_iter_sorted_exact_genomic() {
        let query = GenomicInterval::new(3, 17, 27);
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 15, 25),
            GenomicInterval::new(3, 10, 20), // bounded, but missing overlap req
            GenomicInterval::new(3, 15, 25), // bounded, but missing overlap req
            GenomicInterval::new(3, 20, 30), // first and last
            GenomicInterval::new(3, 40, 50), // unbounded
            GenomicInterval::new(4, 10, 20),
            GenomicInterval::new(4, 25, 35),
        ];
        let set = GenomicIntervalSet::from_sorted(intervals).unwrap();
        let mut overlaps = set
            .find_iter_sorted_exact(&query, 7)
            .unwrap()
            .into_iter()
            .cloned();
        let first = overlaps.next().unwrap();
        let last = overlaps.last();
        assert_eq!(first, GenomicInterval::new(3, 20, 30));
        assert!(last.is_none());
    }

    #[test]
    fn find_query_frac_a() {
        let query = Interval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            Interval::new(0, 10),
            Interval::new(5, 15), // first
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
            Interval::new(17, 27), // bounded, but missing overlap req
            Interval::new(20, 30),
        ];
        let expected = vec![
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_query_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_query_frac_b() {
        let query = Interval::new(10, 20);
        let frac = 0.2;
        let intervals = vec![
            Interval::new(0, 10),
            Interval::new(5, 15), // first
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
            Interval::new(17, 27), // last
            Interval::new(20, 30),
        ];
        let expected = vec![
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
            Interval::new(17, 27),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_query_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_query_frac_c() {
        let query = Interval::new(10, 20);
        let frac = 1.0;
        let intervals = vec![
            Interval::new(0, 10),
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20), // only
            Interval::new(15, 25),
            Interval::new(17, 27),
            Interval::new(20, 30),
        ];
        let expected = vec![Interval::new(10, 20)];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_query_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_iter_sorted_query_frac() {
        let query = Interval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            Interval::new(0, 10),
            Interval::new(5, 15), // first
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
            Interval::new(17, 27), // bounded, but missing overlap req
            Interval::new(20, 30),
        ];
        let expected = vec![
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20),
            Interval::new(15, 25),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_iter_sorted_query_frac(&query, frac).unwrap();
        for (i, j) in overlaps.into_iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_target_frac_a() {
        let query = Interval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            Interval::new(2, 12), // bounded, but missing overlap req
            Interval::new(5, 15), // first
            Interval::new(7, 17),
            Interval::new(7, 37),  // bounded, but missing overlap req
            Interval::new(10, 20), // last
            Interval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_target_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_target_frac_b() {
        let query = Interval::new(10, 20);
        let frac = 1.0;
        let intervals = vec![
            Interval::new(2, 12),  // bounded, but missing overlap req
            Interval::new(5, 15),  // bounded, but missing overlap req
            Interval::new(7, 17),  // bounded, but missing overlap req
            Interval::new(7, 37),  // bounded, but missing overlap req
            Interval::new(10, 20), // only
            Interval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![Interval::new(10, 20)];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_target_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_target_frac_c() {
        let query = Interval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            Interval::new(8, 18), // bounded, but missing overlap req
            Interval::new(9, 19), // first
            Interval::new(10, 20),
            Interval::new(11, 21), // last
            Interval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            Interval::new(9, 19),
            Interval::new(10, 20),
            Interval::new(11, 21),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_target_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_iter_sorted_target_frac() {
        let query = Interval::new(10, 20);
        let frac = 0.5;
        let intervals = vec![
            Interval::new(2, 12), // bounded, but missing overlap req
            Interval::new(5, 15), // first
            Interval::new(7, 17),
            Interval::new(7, 37),  // bounded, but missing overlap req
            Interval::new(10, 20), // last
            Interval::new(12, 22), // bounded, but missing overlap req
        ];
        let expected = vec![
            Interval::new(5, 15),
            Interval::new(7, 17),
            Interval::new(10, 20),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_iter_sorted_target_frac(&query, frac).unwrap();
        for (i, j) in overlaps.into_iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_reciprocal_frac_a() {
        let query = Interval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            Interval::new(8, 18),
            // overlaps by 90% of target and query
            Interval::new(9, 19), // only
            // overlaps by 90% of query but not target
            Interval::new(9, 20),
            // overlaps by >90% of target but not query
            Interval::new(15, 18),
            // outside interval
            Interval::new(20, 30),
        ];
        let expected = vec![Interval::new(9, 19)];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_reciprocal_frac(&query, frac).unwrap();
        for (i, j) in overlaps.records().iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_iter_sorted_reciprocal_frac_a() {
        let query = Interval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            Interval::new(8, 18),
            // overlaps by 90% of target and query
            Interval::new(9, 19), // only
            // overlaps by 90% of query but not target
            Interval::new(9, 20),
            // overlaps by >90% of target but not query
            Interval::new(15, 18),
            // outside interval
            Interval::new(20, 30),
        ];
        let expected = vec![Interval::new(9, 19)];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_iter_sorted_reciprocal_frac(&query, frac).unwrap();
        for (i, j) in overlaps.into_iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_reciprocal_frac_either_a() {
        let query = Interval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            Interval::new(8, 18),
            // overlaps by 90% of target and query
            Interval::new(9, 19), // first
            // overlaps by 90% of query but not target
            Interval::new(9, 20),
            // overlaps by >90% of target but not query
            Interval::new(15, 18), // last
            // outside interval
            Interval::new(20, 30),
        ];
        let expected = vec![
            Interval::new(9, 19),
            Interval::new(9, 20),
            Interval::new(15, 18),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_reciprocal_frac_either(&query, frac).unwrap();
        for (i, j) in overlaps.records().into_iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_iter_sorted_reciprocal_frac_either_a() {
        let query = Interval::new(10, 20);
        let frac = 0.9;
        let intervals = vec![
            // overlaps by 80% of target
            Interval::new(8, 18),
            // overlaps by 90% of target and query
            Interval::new(9, 19), // first
            // overlaps by 90% of query but not target
            Interval::new(9, 20),
            // overlaps by >90% of target but not query
            Interval::new(15, 18), // last
            // outside interval
            Interval::new(20, 30),
        ];
        let expected = vec![
            Interval::new(9, 19),
            Interval::new(9, 20),
            Interval::new(15, 18),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let overlaps = set.find_iter_sorted_reciprocal_frac(&query, frac).unwrap();
        for (i, j) in overlaps.into_iter().zip(expected.iter()) {
            assert!(i.eq(j))
        }
    }

    #[test]
    fn find_query_frac_unbounded() {
        let query = Interval::new(10, 20);
        let set = IntervalSet::from_sorted(vec![Interval::new(0, 10)]).unwrap();
        assert!(set.find_query_frac(&query, 0.0).is_err());
        assert!(set.find_query_frac(&query, 1.01).is_err());
    }

    #[test]
    fn find_target_frac_unbounded() {
        let query = Interval::new(10, 20);
        let set = IntervalSet::from_sorted(vec![Interval::new(0, 10)]).unwrap();
        assert!(set.find_target_frac(&query, 0.0).is_err());
        assert!(set.find_target_frac(&query, 1.01).is_err());
    }

    #[test]
    fn find_reciprocal_frac_unbounded() {
        let query = Interval::new(10, 20);
        let set = IntervalSet::from_sorted(vec![Interval::new(0, 10)]).unwrap();
        assert!(set.find_reciprocal_frac(&query, 0.0).is_err());
        assert!(set.find_reciprocal_frac(&query, 1.01).is_err());
    }
}
