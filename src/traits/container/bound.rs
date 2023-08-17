use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    Container,
};

/// Identifies the lower bound on a [Container] via a binary tree search
pub trait Bound<C, T, I>: Container<C, T, I>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Identifies the lower bound on the [Container] via a binary tree search
    /// for a provided query.
    ///
    /// This first checks if the [Container] is sorted
    ///
    /// Then, it performs a binary tree search for the lower bound
    /// but performs a biased comparison to search for the lower bound
    /// subtracting the largest possible interval size.
    ///
    /// ## On base coordinates
    ///
    /// ```
    /// use bedrs::{Bound, Container, Interval, IntervalSet};
    ///
    /// let records = vec![
    ///     Interval::new(0, 10),
    ///     Interval::new(10, 20), // <- min
    ///     Interval::new(20, 30),
    ///     Interval::new(30, 40),
    ///     Interval::new(40, 50),
    ///     Interval::new(50, 60),
    /// ];
    /// let query = Interval::new(17, 27);
    /// let mut set = IntervalSet::new(records);
    /// set.sort();
    /// let bound = set.lower_bound(&query);
    /// assert_eq!(bound, Ok(1));
    /// ```
    ///
    /// ## On genomic coordinates
    ///
    /// ```
    /// use bedrs::{Bound, Container, GenomicInterval, GenomicIntervalSet};
    ///
    /// let records = vec![
    ///     GenomicInterval::new(1, 10, 20),
    ///     GenomicInterval::new(2, 10, 20),
    ///     GenomicInterval::new(3, 10, 20), // <- min
    ///     GenomicInterval::new(3, 20, 20),
    ///     GenomicInterval::new(3, 30, 20),
    ///     GenomicInterval::new(4, 10, 20),
    /// ];
    /// let mut set = GenomicIntervalSet::new(records);
    /// set.sort();
    /// let query = GenomicInterval::new(3, 10, 20);
    /// let bound = set.lower_bound(&query);
    /// assert_eq!(bound, Ok(2));
    /// ```
    fn lower_bound(&self, query: &I) -> Result<usize, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            } else if self.max_len().is_none() {
                return Err(SetError::MissingMaxLen);
            }
            Ok(self.lower_bound_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Identifies the lower bound on the [Container] via a binary tree search
    /// for a provided query.
    ///
    /// Does not perform a check if it is sorted beforehand.
    /// Use at your own risk.
    ///
    /// ## On base coordinates
    ///
    /// ```
    /// use bedrs::{Bound, Interval, IntervalSet};
    ///
    /// let records = vec![
    ///     Interval::new(0, 10),
    ///     Interval::new(10, 20), // <- min
    ///     Interval::new(20, 30),
    ///     Interval::new(30, 40),
    ///     Interval::new(40, 50),
    ///     Interval::new(50, 60),
    /// ];
    /// let query = Interval::new(17, 27);
    /// let set = IntervalSet::new(records);
    /// let bound = set.lower_bound_unchecked(&query);
    /// assert_eq!(bound, 1);
    /// ```
    ///
    /// ## On genomic coordinates
    ///
    /// ```
    /// use bedrs::{Bound, GenomicInterval, GenomicIntervalSet};
    ///
    /// let records = vec![
    ///     GenomicInterval::new(1, 10, 20),
    ///     GenomicInterval::new(2, 10, 20),
    ///     GenomicInterval::new(3, 10, 20), // <- min
    ///     GenomicInterval::new(3, 20, 20),
    ///     GenomicInterval::new(3, 30, 20),
    ///     GenomicInterval::new(4, 10, 20),
    /// ];
    /// let set = GenomicIntervalSet::new(records);
    /// let query = GenomicInterval::new(3, 10, 20);
    /// let bound = set.lower_bound_unchecked(&query);
    /// assert_eq!(bound, 2);
    /// ```
    fn lower_bound_unchecked(&self, query: &I) -> usize {
        let mut high = self.len();
        let mut low = 0;
        let max_len = self
            .max_len()
            .expect("max_len is None - is this an empty set?");
        while high > 0 {
            let mid = high / 2;
            let top_half = high - mid;
            let low_index = low + mid;
            let top_index = low + top_half;
            let test_interval = &self.records()[low_index];
            high = mid;
            low = if test_interval.biased_lt(query, max_len) {
                top_index
            } else {
                low
            };
        }
        low
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// with the query. Can result in an error if the [Container] is not sorted.
    fn chr_bound(&self, query: &I) -> Result<Option<usize>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.chr_bound_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// with the query and is upstream. Can result in an error if the [Container]
    /// is not sorted.
    ///
    /// Will return `None` if no record shares a chromosome with the query and is
    /// upstream.
    fn chr_bound_upstream(&self, query: &I) -> Result<Option<usize>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.chr_bound_upstream_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// with the query and is downstream. Can result in an error if the [Container]
    /// is not sorted.
    ///
    /// Will return `None` if no record shares a chromosome with the query and is
    /// downstream.
    fn chr_bound_downstream(&self, query: &I) -> Result<Option<usize>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.chr_bound_downstream_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// with the query. Does not perform a check if it is sorted beforehand.
    /// Use at your own risk.
    fn chr_bound_unchecked(&self, query: &I) -> Option<usize> {
        let mut high = self.len();
        let mut low = 0;
        while high > 0 {
            let mid = high / 2;
            let top_half = high - mid;
            let low_index = low + mid;
            let top_index = low + top_half;
            let test_interval = &self.records()[low_index];
            high = mid;
            low = if test_interval.chr() < query.chr() {
                top_index
            } else {
                low
            };
        }

        // If the low index is the length of the set, then the query is
        // greater than all records in the set.
        if low == self.len() {
            None

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        } else if low == 0 {
            // If the first record in the set has the same chromosome as the
            // query, then return 0.
            if self.records()[0].chr() == query.chr() {
                Some(0)

            // Otherwise, the query is less than all records in the set.
            } else {
                None
            }
        }
        // If the low index is not 0 or the length of the set, then the query
        // shares a chromosome with at least one record in the set.
        // Returns the earliest index of a record with the same chromosome
        else {
            Some(low)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// and is upstream of the query. Does not perform a check if it is
    /// sorted beforehand. Use at your own risk.
    fn chr_bound_upstream_unchecked(&self, query: &I) -> Option<usize> {
        let mut high = self.len();
        let mut low = 0;
        while high > 0 {
            let mid = high / 2;
            let top_half = high - mid;
            let low_index = low + mid;
            let top_index = low + top_half;
            let test_interval = &self.records()[low_index];
            high = mid;
            low = if test_interval.lt(query) {
                if test_interval.chr() == query.chr() {
                    low
                } else {
                    top_index
                }
            } else {
                low
            };
        }

        // If the low index is the length of the set, then the query is
        // greater than all records in the set.
        if low == self.len() {
            None

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        } else if low == 0 {
            // If the first record in the set has the same chromosome as the
            // query, then return 0.
            if self.records()[0].chr() == query.chr() {
                Some(0)

            // Otherwise, the query is less than all records in the set.
            } else {
                None
            }

        // If the resulting record is greater than the query, then the query
        // is less than all records in the set that match its chromosome.
        } else if self.records()[low].gt(query) {
            None
        }
        // If the low index is not 0 or the length of the set, then the query
        // shares a chromosome with at least one record in the set.
        // Returns the earliest index of a record with the same chromosome
        else {
            Some(low)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// and is downstream of the query. Does not perform a check if it is
    /// sorted beforehand. Use at your own risk.
    fn chr_bound_downstream_unchecked(&self, query: &I) -> Option<usize> {
        let mut high = self.len();
        let mut low = 0;
        while high > 0 {
            let mid = high / 2;
            let top_half = high - mid;
            let low_index = low + mid;
            let top_index = low + top_half;
            let test_interval = &self.records()[low_index];
            high = mid;
            low = if test_interval.chr() < query.chr() {
                top_index
            } else if test_interval.chr() == query.chr() && test_interval.gt(query) {
                low
            } else {
                top_index
            };
        }

        // If the low index is the length of the set, then the query is
        // greater than all records in the set.
        if low == self.len() {
            None

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        } else if low == 0 {
            // If the first record in the set has the same chromosome as the
            // query, then return 0.
            if self.records()[0].chr() == query.chr() {
                Some(0)

            // Otherwise, the query is less than all records in the set.
            } else {
                None
            }
        }
        // If the low index is not 0 or the length of the set, then the query
        // shares a chromosome with at least one record in the set.
        // Returns the earliest index of a record with the same chromosome
        else {
            Some(low)
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::errors::SetError, Bound, Container, GenomicInterval, GenomicIntervalSet, Interval,
        IntervalSet,
    };

    #[test]
    fn bsearch_unsorted_chr() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_unsorted_chr_upstream() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.chr_bound_upstream(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_unsorted_chr_downstream() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.chr_bound_downstream(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_empty_chr() {
        let records = Vec::new();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_empty_chr_upstream() {
        let records = Vec::new();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.chr_bound_upstream(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_empty_chr_downstream() {
        let records = Vec::new();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.chr_bound_downstream(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_base_low() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(0));
    }

    #[test]
    fn bsearch_base_high() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(300, 320);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(251));
    }

    #[test]
    fn bsearch_base_mid() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(200, 220);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(151));
    }

    #[test]
    fn bsearch_base_containing() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(0, 500);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(0));
    }

    #[test]
    fn bsearch_genomic_low() {
        let records = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 10, 20),
            GenomicInterval::new(3, 10, 20), // <- min
            GenomicInterval::new(3, 20, 20),
            GenomicInterval::new(3, 30, 20),
            GenomicInterval::new(4, 10, 20),
        ];
        let mut set = GenomicIntervalSet::new(records);
        set.sort();
        let query = GenomicInterval::new(3, 10, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(2));
    }

    #[test]
    fn bsearch_genomic_high() {
        let records = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 10, 20),
            GenomicInterval::new(3, 10, 20),
            GenomicInterval::new(3, 20, 20), // <- min
            GenomicInterval::new(3, 30, 40),
            GenomicInterval::new(4, 10, 20),
        ];
        let mut set = GenomicIntervalSet::new(records);
        set.sort();
        let query = GenomicInterval::new(3, 25, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Ok(3));
    }

    #[test]
    fn bsearch_unsorted() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Err(SetError::UnsortedSet));
    }

    #[test]
    fn bsearch_equality() {
        let records = vec![
            Interval::new(10, 20),
            Interval::new(20, 30), // <- min
            Interval::new(30, 40),
            Interval::new(40, 50),
            Interval::new(50, 60),
        ];
        let query = Interval::new(20, 25);
        let set = IntervalSet::new(records);
        let bound = set.lower_bound_unchecked(&query);
        assert_eq!(bound, 1);
    }

    #[test]
    fn bsearch_zero() {
        let records = vec![
            Interval::new(0, 10), // <- min
            Interval::new(10, 20),
            Interval::new(20, 30),
            Interval::new(30, 40),
            Interval::new(40, 50),
            Interval::new(50, 60),
        ];
        let query = Interval::new(5, 20);
        let set = IntervalSet::new(records);
        let bound = set.lower_bound_unchecked(&query);
        assert_eq!(bound, 0);
    }

    #[test]
    fn bsearch_multizero() {
        let records = vec![
            Interval::new(0, 10), // <- min
            Interval::new(0, 10),
            Interval::new(10, 20),
            Interval::new(20, 30),
            Interval::new(30, 40),
            Interval::new(40, 50),
            Interval::new(50, 60),
        ];
        let query = Interval::new(5, 20);
        let set = IntervalSet::new(records);
        let bound = set.lower_bound_unchecked(&query);
        assert_eq!(bound, 0);
    }

    #[test]
    fn bsearch_zero_example() {
        let query = GenomicInterval::new(2, 226, 376);
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300), // <- min
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(2, 53, 353),
            GenomicInterval::new(2, 204, 504),
        ];
        let set = GenomicIntervalSet::new(intervals);
        let bound = set.lower_bound_unchecked(&query);
        assert_eq!(bound, 1);
    }

    #[test]
    fn bsearch_no_max_len() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::from_sorted(records).unwrap();
        let query = Interval::new(10, 20);
        set.max_len_mut().take();
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Err(SetError::MissingMaxLen));
    }

    #[test]
    #[should_panic]
    fn bsearch_no_max_len_unchecked_panic() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::from_sorted(records).unwrap();
        let query = Interval::new(10, 20);
        set.max_len_mut().take();
        set.lower_bound_unchecked(&query);
    }

    #[test]
    fn bsearch_chr_a() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300), // <- min
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound(&query).unwrap();
        assert_eq!(bound, Some(1));
    }

    #[test]
    fn bsearch_chr_b() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300), // <- min
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(3, 16, 316),
            GenomicInterval::new(4, 53, 353),
        ];
        let query = GenomicInterval::new(1, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound(&query).unwrap();
        assert_eq!(bound, Some(0));
    }

    #[test]
    fn bsearch_chr_c() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353), // <- min
        ];
        let query = GenomicInterval::new(3, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound(&query).unwrap();
        assert_eq!(bound, Some(3));
    }

    #[test]
    fn bsearch_chr_d() {
        // no minimum in this set
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(4, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_e() {
        // no minimum in this set
        let intervals = vec![
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(3, 0, 300),
            GenomicInterval::new(4, 16, 316),
            GenomicInterval::new(5, 53, 353),
        ];
        let query = GenomicInterval::new(1, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_upstream_a() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300), // <- min
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(1));
    }

    #[test]
    fn bsearch_chr_upstream_b() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300), // <- min
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 18, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(1));
    }

    #[test]
    fn bsearch_chr_upstream_c() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353), // <- min
        ];
        let query = GenomicInterval::new(2, 53, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(1));
    }

    #[test]
    fn bsearch_chr_upstream_d() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353), // <- min
        ];
        let query = GenomicInterval::new(3, 54, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(3));
    }

    #[test]
    fn bsearch_chr_upstream_e() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ]; // no min
        let query = GenomicInterval::new(3, 50, 52);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_upstream_f() {
        let intervals = vec![
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(3, 0, 300),
            GenomicInterval::new(3, 16, 316),
            GenomicInterval::new(4, 53, 353),
        ]; // no min
        let query = GenomicInterval::new(1, 50, 52);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_upstream_g() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20), // <- min
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 32);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(0));
    }

    #[test]
    fn bsearch_chr_downstream_a() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316), // <- min
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 10, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_downstream(&query).unwrap();
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_chr_downstream_c() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353), // <- min
        ];
        let query = GenomicInterval::new(3, 10, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_downstream(&query).unwrap();
        assert_eq!(bound, Some(3));
    }

    #[test]
    fn bsearch_chr_downstream_d() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316),
            GenomicInterval::new(3, 53, 353),
        ]; // no min
        let query = GenomicInterval::new(3, 54, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_downstream(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_downstream_e() {
        let intervals = vec![
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(3, 0, 300),
            GenomicInterval::new(3, 16, 316),
            GenomicInterval::new(4, 53, 353),
        ]; // no min
        let query = GenomicInterval::new(1, 54, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_downstream(&query).unwrap();
        assert_eq!(bound, None);
    }
}
