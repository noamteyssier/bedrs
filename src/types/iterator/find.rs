use crate::traits::{IntervalBounds, ValueBounds};
use std::marker::PhantomData;

pub enum QueryMethod<T: ValueBounds> 
{
    Compare,
    CompareBy(T),
    CompareExact(T),
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
    fn predicate(&self, interval: &I) -> bool {
        match self.method {
            QueryMethod::Compare => interval.overlaps(self.query),
            QueryMethod::CompareBy(val) => interval.overlaps_by(self.query, val),
            QueryMethod::CompareExact(val) => interval.overlaps_by_exactly(self.query, val),
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
            if self.predicate(interval) {
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
    fn predicate(&self, interval: &I) -> bool {
        match self.method {
            QueryMethod::Compare => interval.overlaps(self.query),
            QueryMethod::CompareBy(val) => interval.overlaps_by(self.query, val),
            QueryMethod::CompareExact(val) => interval.overlaps_by_exactly(self.query, val),
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
            if self.predicate(interval) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}
