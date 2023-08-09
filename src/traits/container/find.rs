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
            .records()
            .iter()
            .filter(|x| x.overlaps(query))
            .map(|x| x.to_owned())
            .collect();
        Self::ContainerType::new(records)
    }

    /// Finds all intervals that overlap a query by some minimum
    /// amount and returns the same `Container` type with all found regions.
    fn find_min(&self, query: &I, minimum: T) -> Self::ContainerType {
        let records = self
            .records()
            .iter()
            .filter(|x| x.overlaps_by(query, minimum))
            .map(|x| x.to_owned())
            .collect();
        Self::ContainerType::new(records)
    }

    /// Finds all intervals that overlap a query by some exact
    /// amount and returns the same `Container` type with all found regions.
    fn find_exact(&self, query: &I, exact: T) -> Self::ContainerType {
        let records = self
            .records()
            .iter()
            .filter(|x| x.overlaps_by_exactly(query, exact))
            .map(|x| x.to_owned())
            .collect();
        Self::ContainerType::new(records)
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
}

#[cfg(test)]
mod testing {
    use super::Find;
    use crate::{
        traits::Container,
        types::{Interval, IntervalSet},
    };

    #[test]
    fn test_find() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find(&query);
        assert_eq!(overlaps.len(), 4);
    }

    #[test]
    fn test_find_minimum() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_min(&query, 5);
        assert_eq!(overlaps.len(), 3);
    }

    #[test]
    fn test_find_exact() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_exact(&query, 7);
        assert_eq!(overlaps.len(), 1);
    }

    #[test]
    fn test_find_iter() {
        let query = Interval::new(5, 12);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let num_overlaps = set.find_iter(&query).count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn test_find_iter_sorted() {
        let query = Interval::new(5, 12);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let num_overlaps = set.find_iter_sorted(&query).unwrap().count();
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn test_find_iter_sorted_wrong_order() {
        let query = Interval::new(5, 12);
        let starts = vec![15, 20, 25, 10];
        let ends = vec![45, 50, 55, 40];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_iter_sorted(&query);
        assert!(overlaps.is_err());
    }

    #[test]
    fn test_find_iter_min() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find_iter_min(&query, 5);
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }

    #[test]
    fn test_find_iter_sorted_min() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let overlaps = set.find_iter_sorted_min(&query, 5).unwrap();
        let num_overlaps = overlaps.count();
        assert_eq!(num_overlaps, 3);
    }
}
