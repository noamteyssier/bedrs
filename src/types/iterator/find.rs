use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::QueryMethod,
};
use std::marker::PhantomData;

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
            let min_overlap = query.f_len(*frac);
            target.overlaps_by(query, min_overlap)
        }
        QueryMethod::CompareByTargetFraction(frac) => {
            let min_overlap = target.f_len(*frac);
            target.overlaps_by(query, min_overlap)
        }
        QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
            let query_min_overlap = query.f_len(*f_query);
            let target_min_overlap = target.f_len(*f_target);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix && target_min_overlap <= ix
            } else {
                false
            }
        }
        QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
            let query_min_overlap = query.f_len(*f_query);
            let target_min_overlap = target.f_len(*f_target);
            if let Some(ix) = target.overlap_size(query) {
                query_min_overlap <= ix || target_min_overlap <= ix
            } else {
                false
            }
        }
    }
}

pub struct FindIterOwned<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    inner: &'a Vec<I>,
    query: Iv,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
    method: QueryMethod<T>,
}
impl<'a, C, T, I, Iv> FindIterOwned<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: Iv, method: QueryMethod<T>) -> Self {
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
impl<'a, C, T, I, Iv> Iterator for FindIterOwned<'a, C, T, I, Iv>
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
            if predicate(interval, &self.query, &self.method) {
                return Some(interval);
            }
        }
        None
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

pub struct FindIterSortedOwned<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T> + 'a,
    Iv: IntervalBounds<C, T> + 'a,
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: Iv,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
    method: QueryMethod<T>,
}
impl<'a, C, T, I, Iv> FindIterSortedOwned<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: Iv, offset: usize, method: QueryMethod<T>) -> Self {
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
impl<'a, C, T, I, Iv> Iterator for FindIterSortedOwned<'a, C, T, I, Iv>
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
            if predicate(interval, &self.query, &self.method) {
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
    use crate::{BaseInterval, Coordinates};

    #[test]
    fn test_f_len_a() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.5;
        let len = iv.f_len(frac);
        assert_eq!(len, 50);
    }

    #[test]
    fn test_f_len_b() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.3;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_c() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.301;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_d() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.299;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }
}
