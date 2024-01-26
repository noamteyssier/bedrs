use crate::traits::{ChromBounds, IntervalBounds, ValueBounds};
use std::marker::PhantomData;

/// An enumeration of the different methods of querying a query
/// interval and a target interval
///
/// TODO: Validate that the query method is valid and remove Result from Find methods
#[derive(Debug, Default, Clone, Copy)]
pub enum QueryMethod<T: ValueBounds> {
    /// Compare the query and target intervals using the `overlaps` method
    #[default]
    Compare,

    /// Compare the query and target intervals using the `overlaps_by` method
    CompareBy(T),

    /// Compare the query and target intervals using the `overlaps_by_exactly` method
    CompareExact(T),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query interval
    CompareByQueryFraction(f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the target interval
    CompareByTargetFraction(f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query and target intervals
    /// respectively and accepting the query only if both of the fractions are met
    CompareReciprocalFractionAnd(f64, f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query and target intervals
    /// respectively and accepting the query if either of the fractions are met
    CompareReciprocalFractionOr(f64, f64),
}

/// Calculate the length of an interval as a fraction of its total length
pub fn f_len<I, C, T>(interval: &I, frac: f64) -> T
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    let len_f: f64 = interval.len().to_f64().unwrap();
    let n = len_f * frac;
    T::from_f64(n.round()).unwrap()
}

/// Determine whether a query interval overlaps a target interval
/// using a specific overlap method
pub fn predicate<I, Iv, C, T>(target: &I, query: &Iv, method: &QueryMethod<T>) -> bool
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
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
        QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
            let query_min_overlap = f_len(query, *f_query);
            let target_min_overlap = f_len(target, *f_target);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix && target_min_overlap <= ix
            } else {
                false
            }
        }
        QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
            let query_min_overlap = f_len(query, *f_query);
            let target_min_overlap = f_len(target, *f_target);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix || target_min_overlap <= ix
            } else {
                false
            }
        }
    }
}

pub struct FindIter<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T> + 'a,
    Iv: IntervalBounds<C, T> + 'a,
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a Iv,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
    method: QueryMethod<T>,
}
impl<'a, C, T, I, Iv> FindIter<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, method: QueryMethod<T>) -> Self {
        Self {
            inner,
            query,
            offset: 0,
            phantom_t: PhantomData,
            phantom_c: PhantomData,
            method,
        }
    }
}
impl<'a, C, T, I, Iv> Iterator for FindIter<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
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

pub struct FindIterSorted<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T> + 'a,
    Iv: IntervalBounds<C, T> + 'a,
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a Iv,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
    method: QueryMethod<T>,
}
impl<'a, C, T, I, Iv> FindIterSorted<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, offset: usize, method: QueryMethod<T>) -> Self {
        Self {
            inner,
            query,
            offset,
            phantom_t: PhantomData,
            phantom_c: PhantomData,
            method,
        }
    }
}
impl<'a, C, T, I, Iv> Iterator for FindIterSorted<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
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
