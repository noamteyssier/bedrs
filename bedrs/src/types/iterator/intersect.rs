use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::Query,
    Intersect,
};
use std::{collections::VecDeque, fmt::Debug, marker::PhantomData};

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
    C: ChromBounds,
    T: ValueBounds,
{
    iter_left: It,
    iter_right: It,
    queue_left: VecDeque<I>,
    queue_right: VecDeque<I>,
    queue_right_matched: VecDeque<I>,
    method: Query<T>,
    phantom_c: PhantomData<C>,
    is_new: bool,
}
impl<It, I, C, T> IntersectIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(iter_left: It, iter_right: It) -> Self {
        Self {
            iter_left,
            iter_right,
            queue_left: VecDeque::new(),
            queue_right: VecDeque::new(),
            queue_right_matched: VecDeque::new(),
            method: Query::default(),
            phantom_c: PhantomData,
            is_new: true,
        }
    }

    pub fn new_with_method(iter_left: It, iter_right: It, method: Query<T>) -> Self {
        Self {
            iter_left,
            iter_right,
            queue_left: VecDeque::new(),
            queue_right: VecDeque::new(),
            queue_right_matched: VecDeque::new(),
            method,
            phantom_c: PhantomData,
            is_new: true,
        }
    }

    fn next_interval_left(&mut self) -> Option<I> {
        if let Some(next) = self.queue_left.pop_front() {
            self.is_new = false;
            Some(next)
        } else {
            self.is_new = true;
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

    fn next_matched_interval(&mut self) -> Option<I> {
        self.queue_right_matched.pop_front()
    }

    fn next_target(&mut self) -> Option<I> {
        if self.is_new {
            if let Some(next) = self.next_matched_interval() {
                Some(next)
            } else {
                self.next_interval_right()
            }
        } else {
            self.next_interval_right()
        }
    }
}

impl<It, I, C, T> Iterator for IntersectIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T> + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let query = self.next_interval_left()?;
        let Some(mut target) = self.next_target() else {
            // if there are no more targets and we have a new query we're done
            if self.is_new {
                return None;
            }
            // otherwise we advance the query and try again
            return self.next();
        };

        loop {
            if self.method.predicate(&target, &query) {
                // find the intersection
                let ix = query.intersect(&target);

                // push the query back onto the queue
                self.queue_left.push_front(query);

                // push the matched target onto the queue
                self.queue_right_matched.push_front(target);

                // return the intersection
                return ix;
            }
            // push the target back onto the queue
            if query.lt(&target) {
                self.queue_right.push_front(target);
                break;
            }
            // keep popping from the right until we find an interval
            // that overlaps, is greater than the query, or we run out
            target = self.next_target()?;
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
        traits::{ChromBounds, IntervalBounds, ValueBounds},
        types::{Query, QueryMethod, StrandMethod},
        BaseInterval, Bed3,
    };

    fn validate_records<I, C, T>(obs: &[I], exp: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ChromBounds,
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
            BaseInterval::new(100, 300),
            BaseInterval::new(400, 475),
            BaseInterval::new(500, 550),
        ];
        let intervals_b = vec![
            BaseInterval::new(120, 160),
            BaseInterval::new(460, 470),
            BaseInterval::new(490, 500),
        ];
        let expected = vec![BaseInterval::new(120, 160), BaseInterval::new(460, 470)];

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
        let intervals_a = vec![BaseInterval::new(100, 300), BaseInterval::new(400, 475)];
        let intervals_b = vec![BaseInterval::new(80, 120), BaseInterval::new(460, 480)];
        let expected = vec![BaseInterval::new(100, 120), BaseInterval::new(460, 475)];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///  x---------y   x------------y
    ///        i------------j
    /// ===============================
    ///        i---y   x----j
    fn intersections_c() {
        let intervals_a = vec![BaseInterval::new(10, 30), BaseInterval::new(40, 60)];
        let intervals_b = vec![BaseInterval::new(20, 50)];
        let expected = vec![BaseInterval::new(20, 30), BaseInterval::new(40, 50)];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///        i------------j
    ///  x---------y   x------------y
    /// ===============================
    ///        i---y   x----j
    fn intersections_d() {
        let intervals_a = vec![BaseInterval::new(20, 50)];
        let intervals_b = vec![BaseInterval::new(10, 30), BaseInterval::new(40, 60)];
        let expected = vec![BaseInterval::new(20, 30), BaseInterval::new(40, 50)];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///  x---------y   x-------------y   x----------y
    ///        i------------j    i------------j
    /// ==============================================
    ///        i---y   x----j    i---y   x-----j
    fn intersections_e() {
        let intervals_a = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(40, 60),
            BaseInterval::new(70, 90),
        ];
        let intervals_b = vec![BaseInterval::new(20, 50), BaseInterval::new(50, 80)];
        let expected = vec![
            BaseInterval::new(20, 30),
            BaseInterval::new(40, 50),
            BaseInterval::new(50, 60),
            BaseInterval::new(70, 80),
        ];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///        i------------j    i------------j
    ///  x---------y   x-------------y   x----------y
    /// ==============================================
    ///        i---y   x----j    i---y   x-----j
    fn intersections_f() {
        let intervals_a = vec![BaseInterval::new(20, 50), BaseInterval::new(50, 80)];
        let intervals_b = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(40, 60),
            BaseInterval::new(70, 90),
        ];
        let expected = vec![
            BaseInterval::new(20, 30),
            BaseInterval::new(40, 50),
            BaseInterval::new(50, 60),
            BaseInterval::new(70, 80),
        ];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    ///  x---------y   x-------------y   x----------y
    ///        i------------j               i----j
    /// ==============================================
    ///        i---y   x----j               i----j
    fn intersections_g() {
        let intervals_a = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(40, 60),
            BaseInterval::new(70, 90),
        ];
        let intervals_b = vec![BaseInterval::new(20, 50), BaseInterval::new(75, 85)];
        let expected = vec![
            BaseInterval::new(20, 30),
            BaseInterval::new(40, 50),
            BaseInterval::new(75, 85),
        ];
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    /// `q_min` = 0.5
    ///     x---------y    x-----------y
    ///     i-----j        i--j
    /// ===================================
    fn intersections_query_fraction() {
        let intervals_a = vec![BaseInterval::new(100, 300), BaseInterval::new(400, 600)];
        let intervals_b = vec![BaseInterval::new(100, 200), BaseInterval::new(400, 450)];
        let expected = vec![BaseInterval::new(100, 200)];
        let frac = 0.5;
        let method = Query::new(
            QueryMethod::CompareByQueryFraction(frac),
            StrandMethod::Ignore,
        );
        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new_with_method(iter_a, iter_b, method);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }

    #[test]
    /// `t_min` = 0.5
    ///     x---------y    x-----------y
    ///     i-----j        i--j
    /// ===================================
    fn intersections_target_fraction() {
        let intervals_a = vec![BaseInterval::new(100, 300), BaseInterval::new(400, 600)];
        let intervals_b = vec![BaseInterval::new(100, 200), BaseInterval::new(400, 450)];
        let expected = vec![BaseInterval::new(100, 200), BaseInterval::new(400, 450)];
        let frac = 0.5;
        let method = Query::new(
            QueryMethod::CompareByTargetFraction(frac),
            StrandMethod::Ignore,
        );
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
            Bed3::new(1, 100, 300),
            Bed3::new(1, 400, 475),
            Bed3::new(1, 500, 550),
        ];
        let intervals_b = vec![
            Bed3::new(1, 120, 160),
            Bed3::new(1, 460, 470),
            Bed3::new(1, 490, 500),
        ];
        let expected = vec![Bed3::new(1, 120, 160), Bed3::new(1, 460, 470)];

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
            Bed3::new(1, 100, 300),
            Bed3::new(2, 400, 475),
            Bed3::new(2, 500, 550),
        ];
        let intervals_b = vec![
            Bed3::new(1, 120, 160),
            Bed3::new(1, 460, 470),
            Bed3::new(1, 490, 500),
        ];
        let expected = vec![Bed3::new(1, 120, 160)];

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
        let intervals_a = vec![Bed3::new(2, 400, 475), Bed3::new(2, 500, 550)];
        let intervals_b = vec![
            Bed3::new(1, 120, 160),
            Bed3::new(2, 460, 470),
            Bed3::new(2, 510, 520),
        ];
        let expected = vec![Bed3::new(2, 460, 470), Bed3::new(2, 510, 520)];

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
            Bed3::new(1, 120, 160),
            Bed3::new(2, 400, 475),
            Bed3::new(2, 500, 550),
        ];
        let intervals_b = vec![Bed3::new(2, 460, 470), Bed3::new(2, 510, 520)];
        let expected = vec![Bed3::new(2, 460, 470), Bed3::new(2, 510, 520)];

        let iter_a = intervals_a.into_iter();
        let iter_b = intervals_b.into_iter();
        let ix_iter = IntersectIter::new(iter_a, iter_b);
        let intersections: Vec<_> = ix_iter.collect();
        validate_records(&intersections, &expected);
    }
}
