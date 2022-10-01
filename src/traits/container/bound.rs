use crate::{
    traits::{IntervalBounds, ValueBounds},
    Container,
};

pub trait Bound<T, I>: Container<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    fn lower_bound(&self, query: &I) -> Option<usize> {
        if self.is_sorted() {
            Some(self.lower_bound_unchecked(query))
        } else {
            None
        }
    }

    fn lower_bound_unchecked(&self, query: &I) -> usize {
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
    use super::Bound;
    use crate::{Container, GenomicInterval, GenomicIntervalSet, Interval, IntervalSet};

    #[test]
    fn bsearch_base_low() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Some(10));
    }

    #[test]
    fn bsearch_base_high() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let mut set = IntervalSet::new(records);
        set.sort();
        let query = Interval::new(300, 320);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Some(300));
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
        assert_eq!(bound, Some(2));
    }

    #[test]
    fn bsearch_genomic_high() {
        let records = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 10, 20),
            GenomicInterval::new(3, 10, 20),
            GenomicInterval::new(3, 20, 20),
            GenomicInterval::new(3, 30, 20), // <- min
            GenomicInterval::new(4, 10, 20),
        ];
        let mut set = GenomicIntervalSet::new(records);
        set.sort();
        let query = GenomicInterval::new(3, 25, 20);
        let bound = set.lower_bound(&query);
        assert_eq!(bound, Some(4));
    }

    #[test]
    fn bsearch_unsorted() {
        let records = (0..500).map(|x| Interval::new(x, x + 50)).collect();
        let set = IntervalSet::new(records);
        let query = Interval::new(10, 20);
        let bound = set.lower_bound(&query);
        assert!(bound.is_none());
    }
}
