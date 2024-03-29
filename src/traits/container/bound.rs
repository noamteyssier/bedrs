use std::cmp::Ordering;

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
        let max_len = self
            .max_len()
            .expect("max_len is None - is this an empty set?");
        self.records()
            .binary_search_by(|iv| {
                if iv.biased_lt(query, max_len) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or_else(|x| x)
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

    /// Finds the latest record in the [Container] that shares a chromosome
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

    /// Finds the latest record in the [Container] that shares a chromosome
    /// and a strand with the query and is upstream. Can result in an error if
    /// the [Container] is not sorted.
    ///
    /// Will return `None` if no record shares a chromosome and a strand with
    /// the query and is upstream.
    fn stranded_upstream_bound(&self, query: &I) -> Result<Option<usize>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.stranded_upstream_bound_unchecked(query))
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
    /// and a strand with the query and is downstream. Can result in an error if
    /// the [Container] is not sorted.
    ///
    /// Will return `None` if no record shares a chromosome and a strand with
    /// the query and is downstream.
    fn stranded_downstream_bound(&self, query: &I) -> Result<Option<usize>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.stranded_downstream_bound_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// with the query. Does not perform a check if it is sorted beforehand.
    /// Use at your own risk.
    fn chr_bound_unchecked(&self, query: &I) -> Option<usize> {
        // Find the partition point for the chromosome
        let bound = self.records().partition_point(|iv| iv.chr() < query.chr());

        // if the partition point is 0, then the first record is the
        // earliest record that shares a chromosome with the query or
        // there are no records that share a chromosome with the query.
        if bound == 0 {
            if self.records()[0].chr() == query.chr() {
                Some(0)
            } else {
                None
            }

        // if the partition point is the length of the records, then
        // the query is potentially greater than all records in the set.
        // If the last record shares a chromosome with the query, then
        // it is the earliest record that shares a chromosome with the
        // query.
        } else if bound == self.len() {
            if self.records()[bound - 1].chr() == query.chr() {
                Some(bound - 1)
            } else {
                None
            }
        } else {
            Some(bound)
        }
    }

    /// Finds the latest record in the [Container] that shares a chromosome
    /// and is upstream of the query. Does not perform a check if it is
    /// sorted beforehand. Use at your own risk.
    fn chr_bound_upstream_unchecked(&self, query: &I) -> Option<usize> {
        // partition point returns the first index in the slice for which
        // the predicate fails (i.e. the index of the first record that is
        // greater than the query).
        let low = self.records().partition_point(|iv| iv.lt(query));

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        if low == 0 {
            let target = &self.records()[0];

            // If the first record in the set has the same chromosome as the
            // query and the start of the record is less than or equal to the
            // start of the query, then return 0.
            if target.chr() == query.chr() && target.start() <= query.start() {
                Some(0)

            // Otherwise, the query is less than all records in the set
            } else {
                None
            }
        } else {
            // otherwise the low index is the index of the first record that
            // is greater than the query. We subtract 1 to get the index of
            // the last record that is less than the query.
            let idx = low - 1;

            // If the record at the index has the same chromosome as the
            // query, then return the index.
            if self.records()[idx].chr() == query.chr() {
                Some(idx)

            // Otherwise, the query is less than all records in the set
            // that share a chromosome.
            } else {
                None
            }
        }
    }

    fn stranded_upstream_bound_unchecked(&self, query: &I) -> Option<usize> {
        // partition point returns the first index in the slice for which
        // the predicate fails (i.e. the index of the first record that is
        // greater than the query).
        let low = self
            .records()
            .partition_point(|iv| iv.lt(query) && iv.bounded_strand(query));

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        if low == 0 {
            let target = &self.records()[0];

            // If the first record in the set has the same chromosome as the
            // query and the start of the record is less than or equal to the
            // start of the query, and they share a strand then return 0.
            if target.chr() == query.chr()
                && target.start() <= query.start()
                && target.strand() == query.strand()
            {
                Some(0)
            } else {
                None
            }
        } else {
            // otherwise the low index is the index of the first record that
            // is greater than the query. We subtract 1 to get the index of
            // the last record that is less than the query.
            let idx = low - 1;

            // If the record at the index has the same chromosome as the
            // query and they share a strand then return the index.
            if self.records()[idx].chr() == query.chr()
                && self.records()[idx].strand() == query.strand()
            {
                Some(idx)

            // Otherwise, the query is less than all records in the set
            // that share a chromosome and strand.
            } else {
                None
            }
        }
    }

    /// Finds the earliest record in the [Container] that shares a chromosome
    /// and is downstream of the query. Does not perform a check if it is
    /// sorted beforehand. Use at your own risk.
    fn chr_bound_downstream_unchecked(&self, query: &I) -> Option<usize> {
        // partition point returns the first index in the slice for which
        // the predicate fails (i.e. the index of the first record that is
        // greater than the query).
        let low = self.records().partition_point(|iv| iv.lt(query));

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
    /// and is downstream of the query and shares a strand. Does not perform a check if it is
    /// sorted beforehand. Use at your own risk.
    fn stranded_downstream_bound_unchecked(&self, query: &I) -> Option<usize> {
        // partition point returns the first index in the slice for which
        // the predicate fails (i.e. the index of the first record that is
        // greater than the query).
        let lt_bound = self.records().partition_point(|iv| iv.lt(query));

        // Iterate from the low bound to the end of the set and find the first
        // record that shares a strand with the query.
        // This will short-circuit on the first record that does not share a
        // chromosome.
        let strand_bound = self.records()[lt_bound..]
            .iter()
            .enumerate()
            .take_while(|(_, iv)| iv.bounded_chr(query))
            .filter(|(_, iv)| iv.bounded_strand(query))
            .next()?
            .0;

        let low = lt_bound + strand_bound;

        // If the low index is the length of the set, then the query is
        // greater than all records in the set.
        if low == self.len() {
            None

        // If the low index is 0, then the query is potentially less than
        // all records in the set.
        } else if low == 0 {
            // If the first record in the set has the same chromosome as the
            // query and shares a strand, then return 0.
            if self.records()[0].chr() == query.chr()
                && self.records()[0].strand() == query.strand()
            {
                Some(0)

            // Otherwise, the query is less than all records in the set.
            } else {
                None
            }
        }
        // If the low index is not 0 or the length of the set, then the query
        // shares a chromosome and strand with at least one record in the set.
        // Returns the earliest index of a record with the same chromosome
        else {
            Some(low)
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::errors::SetError, types::StrandedGenomicIntervalSet, Bound, Container,
        GenomicInterval, GenomicIntervalSet, Interval, IntervalSet, Strand,
        StrandedGenomicInterval,
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
    fn bsearch_unsorted_stranded_upstream() {
        let records = (0..500)
            .map(|x| StrandedGenomicInterval::new(1, x, x + 50, Strand::Forward))
            .collect();
        let set = StrandedGenomicIntervalSet::new(records);
        let query = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let bound = set.stranded_upstream_bound(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_unsorted_stranded_downstream() {
        let records = (0..500)
            .map(|x| StrandedGenomicInterval::new(1, x, x + 50, Strand::Forward))
            .collect();
        let set = StrandedGenomicIntervalSet::new(records);
        let query = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let bound = set.stranded_downstream_bound(&query);
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
    fn bsearch_empty_chr_stranded_upstream() {
        let records = Vec::new();
        let set = StrandedGenomicIntervalSet::new(records);
        let query = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let bound = set.stranded_upstream_bound(&query);
        assert!(bound.is_err());
    }

    #[test]
    fn bsearch_empty_chr_stranded_downstream() {
        let records = Vec::new();
        let set = StrandedGenomicIntervalSet::new(records);
        let query = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
        let bound = set.stranded_downstream_bound(&query);
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
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316), // <- closest
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 100, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_chr_upstream_b() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316), // <- closest
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 18, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_chr_upstream_c() {
        let intervals = vec![
            GenomicInterval::new(1, 0, 300),
            GenomicInterval::new(2, 0, 300),
            GenomicInterval::new(2, 16, 316), // <- closest
            GenomicInterval::new(3, 53, 353),
        ];
        let query = GenomicInterval::new(2, 53, 300);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, Some(2));
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
    fn bsearch_chr_upstream_h() {
        let intervals = vec![
            // no min
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 8, 32);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_upstream(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_upstream_a_stranded() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Forward), // <- closest
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong strand
            StrandedGenomicInterval::new(2, 16, 316, Strand::Unknown), // <- wrong strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 100, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_upstream_bound(&query).unwrap();
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_chr_upstream_b_stranded() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward), // <- closest
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 100, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_upstream_bound(&query).unwrap();
        assert_eq!(bound, Some(1));
    }

    #[test]
    fn bsearch_chr_upstream_c_stranded() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 100, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_upstream_bound(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_chr_upstream_d_stranded() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Reverse), // <- wrong strand
            StrandedGenomicInterval::new(2, 16, 316, Strand::Forward), // <- min
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 100, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_upstream_bound(&query).unwrap();
        assert_eq!(bound, Some(2));
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

    #[test]
    fn bsearch_chr_downstream_f() {
        let intervals = vec![
            GenomicInterval::new(1, 70, 220), // <- min
            GenomicInterval::new(1, 142, 292),
            GenomicInterval::new(1, 154, 304),
        ];
        let query = GenomicInterval::new(1, 21, 71);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let bound = set.chr_bound_downstream(&query).unwrap();
        assert_eq!(bound, Some(0));
    }

    #[test]
    fn bsearch_chr_downstream_range_a() {
        let chrs = (0..100).map(|x| x % 3).collect::<Vec<_>>();
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let mut set = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends).unwrap();
        set.sort();
        let query = GenomicInterval::new(0, 12, 15);
        let bound = set.chr_bound_downstream(&query).unwrap().unwrap();
        assert_eq!(bound, 4);
    }

    #[test]
    fn bsearch_stranded_downstream_a() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Forward), // <- min
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 10, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_downstream_bound(&query).unwrap();
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_stranded_downstream_b() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(2, 116, 316, Strand::Forward), // <- min
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 10, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_downstream_bound(&query).unwrap();
        assert_eq!(bound, Some(3));
    }

    #[test]
    fn bsearch_stranded_downstream_c() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(2, 16, 316, Strand::Unknown), // <- wrong-strand
            StrandedGenomicInterval::new(2, 116, 316, Strand::Forward), // <- min
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 10, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_downstream_bound(&query).unwrap();
        assert_eq!(bound, Some(4));
    }

    #[test]
    fn bsearch_stranded_downstream_d() {
        let intervals = vec![
            StrandedGenomicInterval::new(1, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward),
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(2, 16, 316, Strand::Unknown), // <- wrong-strand
            StrandedGenomicInterval::new(2, 116, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(2, 10, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_downstream_bound(&query).unwrap();
        assert_eq!(bound, None);
    }

    #[test]
    fn bsearch_stranded_downstream_e() {
        let intervals = vec![
            StrandedGenomicInterval::new(2, 0, 300, Strand::Forward), // <- wrong-chr
            StrandedGenomicInterval::new(2, 16, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(2, 16, 316, Strand::Unknown), // <- wrong-strand
            StrandedGenomicInterval::new(2, 116, 316, Strand::Reverse), // <- wrong-strand
            StrandedGenomicInterval::new(3, 53, 353, Strand::Forward),
        ];
        let query = StrandedGenomicInterval::new(1, 10, 300, Strand::Forward);
        let set = StrandedGenomicIntervalSet::from_unsorted(intervals);
        let bound = set.stranded_downstream_bound(&query).unwrap();
        assert_eq!(bound, None);
    }
}
