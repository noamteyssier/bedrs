use crate::traits::{ChromBounds, IntervalBounds};
use std::{collections::VecDeque, marker::PhantomData};

/// An iterator that merges overlapping intervals
///
/// This iterator takes an iterator of intervals and merges overlapping
/// intervals. The intervals must be sorted by chromosome and start position.
///
/// Undefined behavior if the intervals are not sorted.
///
/// # Example
///
/// ```
/// use bedrs::{Bed3, MergeIter};
///
/// let intervals = vec![
///     bed3![1, 1, 10],
///     bed3![1, 2, 5],
///     bed3![1, 3, 22],
///     bed3![1, 25, 40],
///     bed3![1, 30, 50],
///     bed3![2, 1, 60],
///     bed3![2, 2, 70],
/// ];
/// let iter = MergeIter::new(intervals.into_iter());
/// let merged: Vec<_> = iter.collect();
/// assert_eq!(merged.len(), 3);
/// ```
pub struct MergeIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    iter: It,
    queue: VecDeque<I>,
    phantom_c: PhantomData<C>,
}
impl<It, I, C> MergeIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(iter: It) -> Self {
        Self {
            iter,
            queue: VecDeque::new(),
            phantom_c: PhantomData,
        }
    }
    fn next_interval(&mut self) -> Option<I> {
        if let Some(next) = self.queue.pop_front() {
            Some(next)
        } else {
            self.iter.next().map(|interval| {
                self.queue.push_back(interval);
                self.queue.pop_front().unwrap()
            })
        }
    }
}
impl<It, I, C> Iterator for MergeIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let mut iv = self.next_interval()?;
        while let Some(next) = self.next_interval() {
            if next.overlaps(&iv) || next.borders(&iv) {
                let new_min = iv.start().min(next.start());
                let new_max = iv.end().max(next.end());
                iv.update_endpoints(&new_min, &new_max);
            } else {
                self.queue.push_back(next);
                break;
            }
        }
        Some(iv)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{bed3, BaseInterval, Bed3, Coordinates};

    #[test]
    fn merge_iter_base() {
        let intervals = vec![
            BaseInterval::new(1, 5),
            BaseInterval::new(2, 4),
            BaseInterval::new(3, 6),
            BaseInterval::new(7, 10),
            BaseInterval::new(8, 12),
        ];
        let expected = [BaseInterval::new(1, 6), BaseInterval::new(7, 12)];
        let interval_iter = intervals.into_iter();
        let merge_iter = MergeIter::new(interval_iter);
        let result: Vec<BaseInterval> = merge_iter.collect();
        assert_eq!(result.len(), expected.len());
        for (res, exp) in result.iter().zip(expected.iter()) {
            assert!(res.eq(exp));
        }
    }

    #[test]
    fn merge_iter_genomic() {
        let intervals = vec![
            bed3![1, 5, 10],
            bed3![1, 10, 14],
            bed3![1, 7, 15],
            bed3![1, 22, 30],
            bed3![1, 25, 35],
            bed3![2, 5, 10],
            bed3![2, 7, 15],
        ];
        let expected = [bed3![1, 5, 15], bed3![1, 22, 35], bed3![2, 5, 15]];
        let interval_iter = intervals.into_iter();
        let merge_iter = MergeIter::new(interval_iter);
        let result: Vec<Bed3<u32>> = merge_iter.collect();
        assert_eq!(result.len(), expected.len());
        for (res, exp) in result.iter().zip(expected.iter()) {
            assert!(Coordinates::eq(res, exp));
        }
    }
}
