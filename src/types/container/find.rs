// use super::Container;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    types::{
        FindIter, FindIterOwned, FindIterSorted, FindIterSortedOwned, IntervalContainer,
        QueryMethod,
    },
};
use anyhow::Result;

/// A trait to query set overlaps through a container
impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn find_method<Iv>(
        &self,
        query: &Iv,
        method: QueryMethod<T>,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        match method {
            QueryMethod::Compare => Ok(self.find(query)),
            QueryMethod::CompareBy(minimum) => Ok(self.find_min(query, minimum)),
            QueryMethod::CompareExact(exact) => Ok(self.find_exact(query, exact)),
            QueryMethod::CompareByQueryFraction(frac) => self.find_query_frac(query, frac),
            QueryMethod::CompareByTargetFraction(frac) => self.find_target_frac(query, frac),
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
                self.find_reciprocal_frac_either(query, f_query, f_target)
            }
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
                self.find_reciprocal_frac(query, f_query, f_target)
            }
        }
    }

    /// Finds all intervals that overlap a query and returns
    /// the same `Container` type with all found regions.
    #[must_use]
    pub fn find<Iv>(&self, query: &Iv) -> IntervalContainer<I, C, T>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = self.find_iter(query).cloned().collect::<Vec<I>>();
        IntervalContainer::new(records)
    }

    /// Finds all intervals that overlap a query by some minimum
    /// amount and returns the same `Container` type with all found regions.
    #[must_use]
    pub fn find_min<Iv>(&self, query: &Iv, minimum: T) -> IntervalContainer<I, C, T>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = self
            .find_iter_min(query, minimum)
            .cloned()
            .collect::<Vec<I>>();
        IntervalContainer::new(records)
    }

    /// Finds all intervals that overlap a query by some exact
    /// amount and returns the same `Container` type with all found regions.
    #[must_use]
    pub fn find_exact<Iv>(&self, query: &Iv, exact: T) -> IntervalContainer<I, C, T>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = self
            .find_iter_exact(query, exact)
            .cloned()
            .collect::<Vec<I>>();
        IntervalContainer::new(records)
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of the query length and returns the same `Container` type with all found regions.
    pub fn find_query_frac<Iv>(
        &self,
        query: &Iv,
        frac: f64,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = match self.find_iter_query_frac(query, frac) {
            Ok(iter) => iter.into_iter().cloned().collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(IntervalContainer::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of the target length and returns the same `Container` type with all found regions.
    pub fn find_target_frac<Iv>(
        &self,
        query: &Iv,
        frac: f64,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = match self.find_iter_target_frac(query, frac) {
            Ok(iter) => iter.into_iter().cloned().collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(IntervalContainer::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of **both** the query and target lengths and returns the
    /// same `Container` type with all found regions.
    pub fn find_reciprocal_frac<Iv>(
        &self,
        query: &Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = match self.find_iter_reciprocal_frac(query, f_query, f_target) {
            Ok(iter) => iter.into_iter().cloned().collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(IntervalContainer::new(records))
    }

    /// Finds all intervals that overlap a query by some fraction
    /// of **either** the query and target lengths and returns the
    /// same `Container` type with all found regions.
    pub fn find_reciprocal_frac_either<Iv>(
        &self,
        query: &Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<IntervalContainer<I, C, T>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        let records = match self.find_iter_reciprocal_frac_either(query, f_query, f_target) {
            Ok(iter) => iter.into_iter().cloned().collect::<Vec<I>>(),
            Err(e) => return Err(e),
        };
        Ok(IntervalContainer::new(records))
    }

    /// Creates an iterator that finds all overlapping regions
    ///
    /// Does not assume a sorted Container
    pub fn find_iter<'a, Iv>(&'a self, query: &'a Iv) -> FindIter<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        FindIter::new(self.records(), query, QueryMethod::Compare)
    }

    /// Creates an iterator that finds all overlapping regions
    ///
    /// Does not assume a sorted Container
    pub fn find_iter_owned<Iv>(&self, query: Iv) -> FindIterOwned<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        FindIterOwned::new(self.records(), query, QueryMethod::Compare)
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some minimum overlap
    ///
    /// Does not assume a sorted Container
    pub fn find_iter_min<'a, Iv>(&'a self, query: &'a Iv, minimum: T) -> FindIter<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        FindIter::new(self.records(), query, QueryMethod::CompareBy(minimum))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some exact overlap
    ///
    /// Does not assume a sorted Container
    pub fn find_iter_exact<'a, Iv>(&'a self, query: &'a Iv, exact: T) -> FindIter<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        FindIter::new(self.records(), query, QueryMethod::CompareExact(exact))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of the query length
    ///
    /// Does not assume a sorted Container
    pub fn find_iter_query_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIter<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_target_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIter<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_reciprocal_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIter<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if f_query <= 0.0 || f_query > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_query });
        } else if f_target <= 0.0 || f_target > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_target });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target),
        ))
    }

    /// Creates an iterator that finds all overlapping regions
    /// by some fraction of **either** the query and target length
    ///
    /// Does not assume a sorted Container
    pub fn find_iter_reciprocal_frac_either<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIter<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if f_query <= 0.0 || f_query > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_query });
        } else if f_target <= 0.0 || f_target > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_target });
        }
        Ok(FindIter::new(
            self.records(),
            query,
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target),
        ))
    }

    /// Creates a Result Iterator that finds all overlapping regions
    ///
    /// First checks to see if container is sorted
    pub fn find_iter_sorted<'a, Iv>(
        &'a self,
        query: &'a Iv,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_min<'a, Iv>(
        &'a self,
        query: &'a Iv,
        minimum: T,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_exact<'a, Iv>(
        &'a self,
        query: &'a Iv,
        exact: T,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_query_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_target_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_reciprocal_frac<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_reciprocal_frac_unchecked(query, f_query, f_target)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates a Result Iterator that finds all overlapping regions
    /// by some fraction of **both** the query and target length
    ///
    /// First checks to see if container is sorted
    pub fn find_iter_sorted_reciprocal_frac_either<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if self.is_sorted() {
            Ok(self.find_iter_sorted_reciprocal_frac_either_unchecked(query, f_query, f_target)?)
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Creates an Iterator that finds all overlapping regions
    ///
    /// Assumes a sorted Container.
    pub fn find_iter_sorted_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
    ) -> FindIterSorted<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::Compare,
        )
    }

    /// Creates an Iterator that finds all overlapping regions
    ///
    /// Assumes a sorted Container.
    pub fn find_iter_sorted_owned_unchecked<Iv>(
        &self,
        query: Iv,
    ) -> FindIterSortedOwned<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
        let offset = self.lower_bound_unchecked(&query);
        FindIterSortedOwned::new(self.records(), query, offset, QueryMethod::Compare)
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some minimum overlap
    ///
    /// Assumes a sorted Container.
    pub fn find_iter_sorted_min_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        minimum: T,
    ) -> FindIterSorted<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_exact_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        exact: T,
    ) -> FindIterSorted<'_, C, T, I, Iv>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_query_frac_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_target_frac_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        frac: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
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
    pub fn find_iter_sorted_reciprocal_frac_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if f_query <= 0.0 || f_query > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_query });
        } else if f_target <= 0.0 || f_target > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_target });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target),
        ))
    }

    /// Creates an Iterator that finds all overlapping regions
    /// by some fraction of **either** the query and target length
    ///
    /// Assumes a sorted Container.
    pub fn find_iter_sorted_reciprocal_frac_either_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        f_query: f64,
        f_target: f64,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        if f_query <= 0.0 || f_query > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_query });
        } else if f_target <= 0.0 || f_target > 1.0 {
            return Err(SetError::FractionUnbounded { frac: f_target });
        }
        Ok(FindIterSorted::new(
            self.records(),
            query,
            self.lower_bound_unchecked(query),
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target),
        ))
    }

    /// Creates an Iterator that finds all overlapping regions
    /// given some method of comparison
    pub fn find_iter_sorted_method_unchecked<'a, Iv>(
        &'a self,
        query: &'a Iv,
        method: QueryMethod<T>,
    ) -> Result<FindIterSorted<'_, C, T, I, Iv>, SetError>
    where
        Iv: IntervalBounds<C, T>,
    {
        match method {
            QueryMethod::Compare => {
                let iter = self.find_iter_sorted_unchecked(query);
                Ok(iter)
            }
            QueryMethod::CompareBy(minimum) => {
                let iter = self.find_iter_sorted_min_unchecked(query, minimum);
                Ok(iter)
            }
            QueryMethod::CompareExact(exact) => {
                let iter = self.find_iter_sorted_exact_unchecked(query, exact);
                Ok(iter)
            }
            QueryMethod::CompareByQueryFraction(frac) => {
                let iter = self.find_iter_sorted_query_frac_unchecked(query, frac)?;
                Ok(iter)
            }
            QueryMethod::CompareByTargetFraction(frac) => {
                let iter = self.find_iter_sorted_target_frac_unchecked(query, frac)?;
                Ok(iter)
            }
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
                let iter =
                    self.find_iter_sorted_reciprocal_frac_unchecked(query, f_query, f_target)?;
                Ok(iter)
            }
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
                let iter = self
                    .find_iter_sorted_reciprocal_frac_either_unchecked(query, f_query, f_target)?;
                Ok(iter)
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod testing {
    // use super::Find;
    use crate::{
        traits::{ChromBounds, IntervalBounds, ValueBounds},
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
        let overlaps = set.find(&query);
        assert_eq!(overlaps.len(), 4);
    }

    #[test]
    fn find_containing() {
        let query = BaseInterval::new(0, 100);
        let intervals = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set.find(&query);
        assert_eq!(overlaps.len(), 1);
    }

    #[test]
    fn find_containing_iter_sorted() {
        let query = BaseInterval::new(0, 100);
        let intervals = vec![BaseInterval::new(10, 20)];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let overlaps = set.find_iter_sorted(&query).unwrap();
        let counts = overlaps.count();
        assert_eq!(counts, 1);
    }

    #[test]
    fn find_minimum() {
        let query = BaseInterval::new(17, 27);
        let starts = [10, 15, 20, 25];
        let ends = [40, 45, 50, 55];
        let records = starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| BaseInterval::new(*s, *e))
            .collect::<Vec<BaseInterval<u32>>>();
        let set = IntervalContainer::from_unsorted(records);
        let overlaps = set.find_min(&query, 5);
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
        let overlaps = set.find_exact(&query, 7);
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
        let num_overlaps = set.find_iter(&query).count();
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
        let num_overlaps = set.find_iter_owned(query).count();
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
        let num_overlaps = set.find_iter_sorted(&query).unwrap().count();
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
        let num_overlaps = set.find_iter_sorted_owned_unchecked(query).count();
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
        let overlaps = set.find_iter_sorted(&query);
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
        let overlaps = set.find_iter_min(&query, 5);
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
        let overlaps = set.find_iter_exact(&query, 7);
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
        let overlaps = set.find_iter_sorted_min(&query, 5).unwrap();
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
        let overlaps = set.find_iter_sorted_exact(&query, 7).unwrap();
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
        let mut overlaps = set.find_iter_sorted_min(&query, 5).unwrap().copied();
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
        let mut overlaps = set.find_iter_sorted_exact(&query, 7).unwrap().copied();
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
        let overlaps = set.find_query_frac(&query, frac).unwrap();
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
        let overlaps = set.find_query_frac(&query, frac).unwrap();
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
        let overlaps = set.find_query_frac(&query, frac).unwrap();
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
            .find_iter_sorted_query_frac(&query, frac)
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
        let overlaps = set.find_target_frac(&query, frac).unwrap();
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
        let overlaps = set.find_target_frac(&query, frac).unwrap();
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
        let overlaps = set.find_target_frac(&query, frac).unwrap();
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
            .find_iter_sorted_target_frac(&query, frac)
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
        let overlaps = set.find_reciprocal_frac(&query, frac, frac).unwrap();
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
            .find_iter_sorted_reciprocal_frac(&query, frac, frac)
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
        let overlaps = set.find_reciprocal_frac(&query, f_query, f_target).unwrap();
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
            .find_reciprocal_frac_either(&query, f_query, f_target)
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
        let overlaps = set.find_reciprocal_frac_either(&query, frac, frac).unwrap();
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
            .find_iter_sorted_reciprocal_frac_either(&query, frac, frac)
            .unwrap()
            .copied();
        validate_iter(overlap_iter, &expected);
    }

    #[test]
    fn find_query_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        assert!(set.find_query_frac(&query, 0.0).is_err());
        assert!(set.find_query_frac(&query, 1.01).is_err());
    }

    #[test]
    fn find_target_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        assert!(set.find_target_frac(&query, 0.0).is_err());
        assert!(set.find_target_frac(&query, 1.01).is_err());
    }

    #[test]
    fn find_reciprocal_frac_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        assert!(set.find_reciprocal_frac(&query, 0.0, 0.0).is_err());
        assert!(set.find_reciprocal_frac(&query, 1.01, 1.01).is_err());
    }

    #[test]
    fn find_reciprocal_frac_either_unbounded() {
        let query = BaseInterval::new(10, 20);
        let set = IntervalContainer::from_sorted(vec![BaseInterval::new(0, 10)]).unwrap();
        assert!(set.find_reciprocal_frac_either(&query, 0.0, 0.0).is_err());
        assert!(set.find_reciprocal_frac_either(&query, 1.01, 1.01).is_err());
    }
}
