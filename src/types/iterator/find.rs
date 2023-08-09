use crate::traits::{IntervalBounds, ValueBounds};
use std::marker::PhantomData;

pub enum QueryMethod<T: ValueBounds> {
    Compare,
    CompareBy(T),
    CompareExact(T),
    CompareByQueryFraction(f64),
    CompareByTargetFraction(f64),
    CompareReciprocalFractionAnd(f64),
    CompareReciprocalFractionOr(f64),
}

/// Calculate the length of an interval as a fraction of its total length
pub fn f_len<I, T>(interval: &I, frac: f64) -> T
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    let len_f: f64 = interval.len().to_f64().unwrap();
    let n = len_f * frac;
    T::from_f64(n.round()).unwrap()
}

fn predicate<I, T>(target: &I, query: &I, method: &QueryMethod<T>) -> bool
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    match method {
        QueryMethod::Compare => target.overlaps(query),
        QueryMethod::CompareBy(val) => target.overlaps_by(query, *val),
        QueryMethod::CompareExact(val) => target.overlaps_by_exactly(query, *val),
        QueryMethod::CompareByQueryFraction(frac) => {
            let min_overlap = f_len(query, *frac);
            target.overlaps_by(query, min_overlap)
        }
        QueryMethod::CompareByTargetFraction(frac) => {
            let min_overlap = f_len(target, *frac);
            target.overlaps_by(query, min_overlap)
        }
        QueryMethod::CompareReciprocalFractionAnd(frac) => {
            let query_min_overlap = f_len(query, *frac);
            let target_min_overlap = f_len(target, *frac);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix && target_min_overlap <= ix
            } else {
                false
            }
        }
        QueryMethod::CompareReciprocalFractionOr(frac) => {
            let query_min_overlap = f_len(query, *frac);
            let target_min_overlap = f_len(target, *frac);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix || target_min_overlap <= ix
            } else {
                false
            }
        }
    }
}

pub struct FindIter<'a, T, I>
where
    I: IntervalBounds<T> + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a I,
    offset: usize,
    phantom_t: PhantomData<T>,
    method: QueryMethod<T>,
}
impl<'a, T, I> FindIter<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a I, method: QueryMethod<T>) -> Self {
        Self {
            inner,
            query,
            offset: 0,
            phantom_t: PhantomData,
            method,
        }
    }
}
impl<'a, T, I> Iterator for FindIter<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if predicate(interval, self.query, &self.method) {
                return Some(interval);
            }
        }
        None
    }
}

pub struct FindIterSorted<'a, T, I>
where
    I: IntervalBounds<T> + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a I,
    offset: usize,
    phantom_t: PhantomData<T>,
    method: QueryMethod<T>,
}
impl<'a, T, I> FindIterSorted<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a I, offset: usize, method: QueryMethod<T>) -> Self {
        Self {
            inner,
            query,
            offset,
            phantom_t: PhantomData,
            method,
        }
    }
}
impl<'a, T, I> Iterator for FindIterSorted<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if predicate(interval, self.query, &self.method) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::Interval;

    #[test]
    fn test_f_len_a() {
        let interval = Interval::new(0, 100);
        let frac = 0.5;
        let len = f_len(&interval, frac);
        assert_eq!(len, 50);
    }

    #[test]
    fn test_f_len_b() {
        let interval = Interval::new(0, 100);
        let frac = 0.3;
        let len = f_len(&interval, frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_c() {
        let interval = Interval::new(0, 100);
        let frac = 0.301;
        let len = f_len(&interval, frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_d() {
        let interval = Interval::new(0, 100);
        let frac = 0.299;
        let len = f_len(&interval, frac);
        assert_eq!(len, 30);
    }
}
