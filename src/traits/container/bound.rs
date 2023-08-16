use crate::{
    traits::{errors::SetError, IntervalBounds, ValueBounds},
    Container,
};

/// Identifies the lower bound on a [Container] via a binary tree search
pub trait Bound<C, T, I>: Container<C, T, I>
where
    I: IntervalBounds<C, T>,
    C: ValueBounds,
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
}

#[cfg(test)]
mod testing {
    use crate::{
        traits::errors::SetError, Bound, Container, GenomicInterval, GenomicIntervalSet, Interval,
        IntervalSet,
    };

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
}
