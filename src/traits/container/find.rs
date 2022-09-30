use super::Container;
use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::{FindIter, FindIterSorted},
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

    /// Creates an iterator that finds all overlapping regions
    ///
    /// Does not assume a sorted Container
    fn find_iter<'a>(&'a self, query: &'a I) -> FindIter<'_, T, I> {
        FindIter::new(self.records(), query)
    }

    /// Creates an Optional Iterator that finds all overlapping regions
    ///
    /// First checks to see if container is sorted
    fn find_iter_sorted<'a>(&'a self, query: &'a I) -> Option<FindIterSorted<'_, T, I>> {
        if self.is_sorted() {
            Some(self.find_iter_sorted_unchecked(query))
        } else {
            None
        }
    }

    /// Creates an Iterator that finds all overlapping regions
    ///
    /// Assumes a sorted Container.
    fn find_iter_sorted_unchecked<'a>(&'a self, query: &'a I) -> FindIterSorted<'_, T, I> {
        FindIterSorted::new(self.records(), query)
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
        assert!(overlaps.is_none());
    }
}
