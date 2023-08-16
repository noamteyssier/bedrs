use crate::{
    traits::{IntervalBounds, ValueBounds},
    Intersect,
};
use std::{collections::VecDeque, fmt::Debug, marker::PhantomData};

use super::{find::predicate, QueryMethod};

/// An intersection iterator that operates on two sorted iterators
///
/// This iterator takes two sorted iterators of intervals and returns the
/// intersection of the two iterators. The intervals must be sorted by
/// chromosome and start position.
///
/// Undefined behavior if the intervals are not sorted.
///
/// Also assumes that the intervals WITHIN a set are non-overlapping.
///
/// Works by keeping two queues of intervals, one for each iterator. The
/// intervals are popped from the queue and compared. This will consume
/// all target intervals that precede or overlap the query interval.
pub struct IntersectIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T>,
    C: ValueBounds,
    T: ValueBounds,
{
    iter_left: It,
    iter_right: It,
    queue_left: VecDeque<I>,
    queue_right: VecDeque<I>,
    method: QueryMethod<T>,
    phantom_c: PhantomData<C>,
}
impl<It, I, C, T> IntersectIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T>,
    C: ValueBounds,
    T: ValueBounds,
{
    pub fn new(iter_left: It, iter_right: It) -> Self {
        Self {
            iter_left,
            iter_right,
            queue_left: VecDeque::new(),
            queue_right: VecDeque::new(),
            method: QueryMethod::default(),
            phantom_c: PhantomData,
        }
    }

    pub fn new_with_method(iter_left: It, iter_right: It, method: QueryMethod<T>) -> Self {
        Self {
            iter_left,
            iter_right,
            queue_left: VecDeque::new(),
            queue_right: VecDeque::new(),
            method,
            phantom_c: PhantomData,
        }
    }

    fn next_interval_left(&mut self) -> Option<I> {
        if let Some(next) = self.queue_left.pop_front() {
            Some(next)
        } else {
            self.iter_left.next().map(|interval| {
                self.queue_left.push_back(interval);
                self.queue_left.pop_front().unwrap()
            })
        }
    }

    fn next_interval_right(&mut self) -> Option<I> {
        if let Some(next) = self.queue_right.pop_front() {
            Some(next)
        } else {
            self.iter_right.next().map(|interval| {
                self.queue_right.push_back(interval);
                self.queue_right.pop_front().unwrap()
            })
        }
    }
}

impl<It, I, C, T> Iterator for IntersectIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T> + Debug,
    C: ValueBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let query = self.next_interval_left()?;
        let mut target = self.next_interval_right()?;

        loop {
            if predicate(&target, &query, &self.method) {
                // find the intersection
                let ix = query.intersect(&target);

                // push the query back onto the queue
                self.queue_left.push_front(query);

                // return the intersection
                return ix;
            } else {
                // push the target back onto the queue
                if query.lt(&target) {
                    self.queue_right.push_front(target);
                    break;

                // keep popping from the right until we find an interval
                // that overlaps, is greater than the query, or we run out
                } else {
                    target = self.next_interval_right()?;
                    continue;
                }
            }
        }

        // if we get here, we've exhausted the right iterator
        // so there are no more intersections on the left
        // so we can keep popping from the left until we quit
        self.next()
    }
}

#[cfg(test)]
mod testing {
    use super::IntersectIter;
    use crate::{
        traits::{IntervalBounds, ValueBounds},
        types::iterator::QueryMethod,
        GenomicInterval, Interval,
    };

    fn validate_records<I, C, T>(obs: &[I], exp: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ValueBounds,
        T: ValueBounds,
    {
        assert_eq!(obs.len(), exp.len());
        for (obs, exp) in obs.iter().zip(exp.iter()) {
            assert!(obs.eq(exp));
        }
    }

    #[test]
    ///       x-------y   x----y    x---y
    ///        i---j        i-j   i-j
    ///   ==================================
    ///        i---j        i-j
    fn intersections_a() {
        let intervals_a = vec![
            Interval::new(100, 300),
            Interval::new(400, 475),
            Interval::new(500, 550),
        ];
        let intervals_b = vec![
            Interval::new(120, 160),
            Interval::new(460, 470),
            Interval::new(490, 500),
        ];
        let expected = vec![Interval::new(120, 160), Interval::new(460, 470)];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///       x-------y   x----y
    ///     i---j           i----j
    ///  =========================
    ///       x-j           i--y
    fn intersections_b() {
        let intervals_a = vec![Interval::new(100, 300), Interval::new(400, 475)];
        let intervals_b = vec![Interval::new(80, 120), Interval::new(460, 480)];
        let expected = vec![Interval::new(100, 120), Interval::new(460, 475)];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    /// q_min = 0.5
    ///     x---------y    x-----------y
    ///     i-----j        i--j
    /// ===================================
    fn intersections_query_fraction() {
        let intervals_a = vec![Interval::new(100, 300), Interval::new(400, 600)];
        let intervals_b = vec![Interval::new(100, 200), Interval::new(400, 450)];
        let expected = vec![Interval::new(100, 200)];
        let frac = 0.5;
        let method = QueryMethod::CompareByQueryFraction(frac);
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new_with_method(iter_a, iter_b, method);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    /// t_min = 0.5
    ///     x---------y    x-----------y
    ///     i-----j        i--j
    /// ===================================
    fn intersections_target_fraction() {
        let intervals_a = vec![Interval::new(100, 300), Interval::new(400, 600)];
        let intervals_b = vec![Interval::new(100, 200), Interval::new(400, 450)];
        let expected = vec![Interval::new(100, 200), Interval::new(400, 450)];
        let frac = 0.5;
        let method = QueryMethod::CompareByTargetFraction(frac);
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new_with_method(iter_a, iter_b, method);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///       x-------y   x----y    x---y
    ///        i---j        i-j   i-j
    ///   ==================================
    ///        i---j        i-j
    fn intersections_genomic_a() {
        let intervals_a = vec![
            GenomicInterval::new(1, 100, 300),
            GenomicInterval::new(1, 400, 475),
            GenomicInterval::new(1, 500, 550),
        ];
        let intervals_b = vec![
            GenomicInterval::new(1, 120, 160),
            GenomicInterval::new(1, 460, 470),
            GenomicInterval::new(1, 490, 500),
        ];
        let expected = vec![
            GenomicInterval::new(1, 120, 160),
            GenomicInterval::new(1, 460, 470),
        ];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///   |1|    x-------y  |2|  x----y    x---y
    ///   |1|     i---j           i-j   i-j  |2|
    ///   ==================================
    ///        i---j    
    fn intersections_genomic_b() {
        let intervals_a = vec![
            GenomicInterval::new(1, 100, 300),
            GenomicInterval::new(2, 400, 475),
            GenomicInterval::new(2, 500, 550),
        ];
        let intervals_b = vec![
            GenomicInterval::new(1, 120, 160),
            GenomicInterval::new(1, 460, 470),
            GenomicInterval::new(1, 490, 500),
        ];
        let expected = vec![GenomicInterval::new(1, 120, 160)];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///   |1|               |2|  x----y    x---y
    ///   |1|     i---j     |2|   i-j       i-j  
    ///   =========================================
    ///                           i-j       i-j
    fn intersections_genomic_c() {
        let intervals_a = vec![
            GenomicInterval::new(2, 400, 475),
            GenomicInterval::new(2, 500, 550),
        ];
        let intervals_b = vec![
            GenomicInterval::new(1, 120, 160),
            GenomicInterval::new(2, 460, 470),
            GenomicInterval::new(2, 510, 520),
        ];
        let expected = vec![
            GenomicInterval::new(2, 460, 470),
            GenomicInterval::new(2, 510, 520),
        ];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///   |1|     i---j     |2|  x----y    x---y
    ///   |1|               |2|   i-j       i-j  
    ///   =========================================
    ///                           i-j       i-j
    fn intersections_genomic_d() {
        let intervals_a = vec![
            GenomicInterval::new(1, 120, 160),
            GenomicInterval::new(2, 400, 475),
            GenomicInterval::new(2, 500, 550),
        ];
        let intervals_b = vec![
            GenomicInterval::new(2, 460, 470),
            GenomicInterval::new(2, 510, 520),
        ];
        let expected = vec![
            GenomicInterval::new(2, 460, 470),
            GenomicInterval::new(2, 510, 520),
        ];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }
}
