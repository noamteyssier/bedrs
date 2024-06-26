use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::Query,
};
use std::marker::PhantomData;

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
    method: Query<T>,
}
impl<'a, C, T, I, Iv> FindIter<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, offset: usize, method: Query<T>) -> Self {
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
            if self.method.predicate(interval, self.query) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}

pub struct FindIterEnumerate<'a, C, T, I, Iv>
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
    method: Query<T>,
}
impl<'a, C, T, I, Iv> FindIterEnumerate<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, offset: usize, method: Query<T>) -> Self {
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
impl<'a, C, T, I, Iv> Iterator for FindIterEnumerate<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = (usize, &'a I);
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if self.method.predicate(interval, self.query) {
                return Some((self.offset - 1, interval));
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}

pub struct FindIterOwned<'a, C, T, I, Iv>
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
    method: Query<T>,
}
impl<'a, C, T, I, Iv> FindIterOwned<'a, C, T, I, Iv>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: Iv, offset: usize, method: Query<T>) -> Self {
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
            if self.method.predicate(interval, &self.query) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}
